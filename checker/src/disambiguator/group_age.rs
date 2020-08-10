//! Groups entries by their age data.
//!
//! Uses the Largest Consistent Subset algorithm described in [`LCS`].
//!
//! [`LCS`]: https://iopscience.iop.org/article/10.1088/0026-1394/44/3/005/meta
//!

use itertools_num::linspace;
use opltypes::*;

use crate::compiler::interpolate_age::trace_conflict;
use crate::compiler::interpolate_age::BirthDateRange;
use crate::compiler::interpolate_age::NarrowResult;
use crate::compiler::interpolate_age::BDR_DEFAULT_MAX;
use crate::compiler::interpolate_age::BDR_DEFAULT_MIN;
use crate::{AllMeetData, EntryIndex, LifterMap};

fn calc_distance(bd_range1: &BirthDateRange, x: f32) -> f32 {
    if bd_range1.min.count_days() as f32 > x {
        return bd_range1.min.count_days() as f32 - x;
    } else if (bd_range1.max.count_days() as f32) < x {
        return bd_range1.max.count_days() as f32 - x;
    }
    0.0
}

// Need to account for uncertainty here as well
fn calc_error(bd_range1: &BirthDateRange, x: f32) -> f32 {
    calc_distance(bd_range1, x).powf(2.0)
}

fn bd_range_subset_is_consistent(subset_vec: &[usize], bd_range_vec: &[BirthDateRange]) -> bool {
    let mut bdr = BirthDateRange::default();
    for range_idx in subset_vec {
        if bd_range_vec[*range_idx].max < bdr.min {
            return false;
        } else if bd_range_vec[*range_idx].max < bdr.max {
            bdr.max = bd_range_vec[*range_idx].max
        }

        if bd_range_vec[*range_idx].min > bdr.max {
            return false;
        } else if bd_range_vec[*range_idx].min > bdr.min {
            bdr.min = bd_range_vec[*range_idx].min
        }
    }
    true
}

/// Finds the distance of a point from birthdate ranges and returns them in sorted order.
fn get_sorted_errors(
    x: f32,
    subset_vec: &[usize],
    bd_range_vec: &[BirthDateRange],
) -> Vec<(usize, f32)> {
    let mut ii = 0;
    let mut errors: Vec<(usize, f32)> = subset_vec
        .iter()
        .map(|bd_idx| {
            ii += 1;
            (ii - 1, calc_error(&bd_range_vec[*bd_idx], x))
        })
        .collect();
    errors.sort_by(|a, b| (*a).1.partial_cmp(&(*b).1).unwrap());
    errors
}

/// Checks a vector of sample points to find the largest LCS possible at one of these points.
fn find_lcs_numeric(
    subset_vec: &[usize],
    bd_range_vec: &[BirthDateRange],
    test_vals: &[f32],
) -> Option<Vec<usize>> {
    let mut best_errors;
    let mut _wm = 0.0;

    let mut error_vec = Vec::new();
    for val in test_vals {
        error_vec.push(get_sorted_errors(*val, subset_vec, bd_range_vec));
    }
    for r in (2..subset_vec.len()).rev() {
        let mut error_min = error_vec[0][0..r].iter().map(|(_a, b)| b).sum();
        best_errors = error_vec[0][0..r].to_vec();
        _wm = test_vals[0];

        for ii in 1..test_vals.len() {
            let error: f32 = error_vec[ii][0..r].iter().map(|(_a, b)| b).sum();
            if error < error_min {
                error_min = error;
                best_errors = error_vec[ii][0..r].to_vec();
                _wm = test_vals[ii];
            }
        }
        if error_min == 0.0 {
            return Some(best_errors.iter().map(|(a, _b)| subset_vec[*a]).collect());
        }
    }
    None
}

/// Find the points that could possibly yield a minimum.
///
/// This implements the test point algorithm described in the paper mentioned in
/// the document header with some key changes. BirthDate data is a range, instead
/// of a defined point and so the error metric is defined piecewise with E_k(x)=0
/// when x is inside the range of k. Turning points occur in the regions
/// between the maximum of one region and the minimum of the next adjacent
/// region. The points that must be sampled to find the LCS are therefore the
/// midpoints of these regions and the boundary points of the regions.
///
fn get_test_points(subset_vec: &[usize], bd_range_vec: &[BirthDateRange]) -> Vec<f32> {
    // Calculate the points where the ordering of the error curves changes.
    let mut test_points = Vec::new();

    // Intersections in the error curves occur when we are greater than the end of
    // one range and smaller than the start of a second Start by obtaining a
    // list of ranges sorted by the max of the range
    let mut sorted_ranges = Vec::new();
    for range_idx in subset_vec {
        sorted_ranges.push((
            bd_range_vec[*range_idx].min.count_days() as f32,
            (bd_range_vec[*range_idx].max.count_days() as f32),
        ));
    }
    sorted_ranges.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    // There will be one intersection point per range maximum (I think...)
    // Midpoint of left edge and first breakpoint
    test_points
        .push((sorted_ranges[0].1 + ((sorted_ranges[0].1 + sorted_ranges[1].0) / 2.0)) / 2.0);
    for ii in 0..sorted_ranges.len() - 2 {
        let intersection1 = (sorted_ranges[ii].1 + sorted_ranges[ii + 1].0) / 2.0;
        let intersection2 = (sorted_ranges[ii + 1].1 + sorted_ranges[ii + 2].0) / 2.0;
        test_points.push((intersection1 + intersection2) / 2.0)
    }

    // Midpoint of right edge and last breakpoint
    test_points.push(
        (sorted_ranges[sorted_ranges.len() - 1].0
            + ((sorted_ranges[sorted_ranges.len() - 2].1
                + sorted_ranges[sorted_ranges.len() - 1].0)
                / 2.0))
            / 2.0,
    );

    // This definitely oversamples, need to think carefully about what points to
    // include
    for range_idx in subset_vec {
        test_points.push(bd_range_vec[*range_idx].min.count_days() as f32);
        test_points.push(bd_range_vec[*range_idx].max.count_days() as f32);
    }
    test_points.sort_by(|a, b| a.partial_cmp(b).unwrap());
    test_points.dedup();

    test_points
}

/// LCS algorithm variant.
///
/// Uses intersections between error curves to potentially reduce the number of calls.
fn find_lcs_algebraic(subset_vec: &[usize], bd_range_vec: &[BirthDateRange]) -> Option<Vec<usize>> {
    let mut best_errors;
    let mut _wm = 0.0;

    let test_points = get_test_points(subset_vec, bd_range_vec);

    let mut error_vec = Vec::new();
    for val in &test_points {
        error_vec.push(get_sorted_errors(*val, subset_vec, bd_range_vec));
    }

    for r in (2..subset_vec.len()).rev() {
        let mut error_min = error_vec[0][0..r].iter().map(|(_a, b)| b).sum();
        best_errors = error_vec[0][0..r].to_vec();
        _wm = test_points[0];
        for ii in 1..test_points.len() {
            let error: f32 = error_vec[ii][0..r].iter().map(|(_a, b)| b).sum();
            if error < error_min {
                error_min = error;
                best_errors = error_vec[ii][0..r].to_vec();
                _wm = test_points[ii];
            }
        }
        if error_min == 0.0 {
            return Some(best_errors.iter().map(|(a, _b)| subset_vec[*a]).collect());
        }
    }

    None
}

/// Finds the largest consistent subset (if any) of a BirthDateRange.
fn find_lcs(subset_vec: &[usize], bd_range_vec: &[BirthDateRange]) -> Option<Vec<usize>> {
    if subset_vec.is_empty() {
        return None;
    }

    // LCS of a length 1 vector is the vector.
    if subset_vec.len() == 1 {
        return Some(subset_vec.to_vec());
    }

    // LCS of a consistent vector is the vector.
    if bd_range_subset_is_consistent(subset_vec, bd_range_vec) {
        return Some(subset_vec.to_vec());
    }

    // TODO(mpearce): This is wildly inefficient and needs much brainstorming.
    let mut test_vals: Vec<f32> = Vec::new();
    for range_idx in subset_vec {
        let min_date = bd_range_vec[*range_idx].min.count_days() as f32;
        let max_date = bd_range_vec[*range_idx].max.count_days() as f32;
        let days = max_date as usize - min_date as usize + 1;

        let mut curr_test_vals = linspace::<f32>(min_date, max_date, days)
            .collect();
        test_vals.append(&mut curr_test_vals);
    }

    test_vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
    test_vals.dedup();

    // Estimate the cost of various algorithms, to select the most efficient.
    let numeric_ops =
        (test_vals.len() as f32) * (subset_vec.len() as f32) * (subset_vec.len() as f32).log(2.0);

    // TODO(mpearce): Paper has an O(N^3) complexity, but I think we're O(N^2), need to confirm.
    let algebraic_ops = (bd_range_vec.len() as f32).powf(2.0);

    if numeric_ops < algebraic_ops {
        // This needs to be replaced with a union of ranges based on the BirthDateRange
        // data
        find_lcs_numeric(subset_vec, bd_range_vec, &test_vals)
    } else {
        find_lcs_algebraic(subset_vec, bd_range_vec)
    }
}

/// Groups data by consistent subsets of age data, returns the indices of elements
/// of the different groups acceptable_delta is currently unused, in future it
/// will be used to allow some amount of error in data
pub fn group_by_age(bd_range_vec: &[BirthDateRange], _acceptable_delta: u32) -> Vec<Vec<usize>> {
    let mut all_groups_vec = Vec::new();
    let mut ungrouped_vec = Vec::new();
    let mut blank_vec = Vec::new();

    for (idx, range) in bd_range_vec.iter().enumerate() {
        // If this is a blank range bin it seperately
        if range.min == BDR_DEFAULT_MIN && range.max == BDR_DEFAULT_MAX {
            blank_vec.push(idx as usize);
        } else {
            ungrouped_vec.push(idx as usize);
        }
    }

    if !blank_vec.is_empty() {
        all_groups_vec.push(blank_vec);
    }

    let mut lcs = find_lcs(&ungrouped_vec, bd_range_vec);

    // Add this group to our list of groups and find the LCS of the remaining elements.
    while lcs.is_some() {
        ungrouped_vec.retain(|&x| !lcs.as_ref().unwrap().contains(&x));

        all_groups_vec.push(lcs.unwrap());
        lcs = find_lcs(&ungrouped_vec, bd_range_vec);
    }

    // Then the remaining elements are all singletons
    if !ungrouped_vec.is_empty() {
        for bd_range_idx in ungrouped_vec {
            all_groups_vec.push(vec![bd_range_idx]);
        }
    }

    all_groups_vec
}

/// Prints grouped lifter data, for debugging.
pub fn print_groups(bd_groups: &[Vec<usize>], bd_range_vec: &[BirthDateRange], date_vec: &[Date]) {
    println!("Groupings are:");
    for (ii, group) in bd_groups.iter().enumerate() {
        println!("{}: ", ii);
        for range_idx in group {
            println!(
                "({},{}): {} ",
                bd_range_vec[*range_idx].min, bd_range_vec[*range_idx].max, date_vec[*range_idx]
            );
        }
    }
}

/// Age groupings for entries under a username.
fn group_lifter_data(meetdata: &mut AllMeetData, indices: &[EntryIndex], debug: bool) {
    let mut bd_ranges = Vec::new();
    let mut meet_dates = Vec::new();

    // Convert the lifterdata to a list of BirthDateRange's
    for &index in indices {
        let mut range = BirthDateRange::default();

        // Extract the MeetDate first. Because of the borrow checker, the Meet and Entry
        // structs cannot be referenced simultaneously.
        let mdate: Date = meetdata.get_meet(index).date;

        // Get the MeetPath for more helpful debugging output.
        // Cloning is OK since this is only for a few entries for one lifter.
        let path: Option<String> = if debug {
            Some(meetdata.get_meet(index).path.clone())
        } else {
            None
        };

        let entry = meetdata.get_entry(index);

        // Narrow by BirthDate.
        if let Some(birthdate) = entry.birthdate {
            if range.narrow_by_birthdate(birthdate) == NarrowResult::Conflict {
                trace_conflict(debug, &range, mdate, "BirthDate", &birthdate, &path);
            }
        }

        // Narrow by BirthYearRange.
        // Check that this is an actual BirthYearRange
        // TODO(mpearce): This is hacky and gross, fix this
        if !(entry.birthyearrange.is_default()
            || (entry.birthyearrange.max_year as u32 == mdate.year()
                && entry.birthyearrange.min_year < 1800))
        {
            let byr = entry.birthyearrange;

            if range.narrow_by_birthyear_range(byr) == NarrowResult::Conflict {
                trace_conflict(debug, &range, mdate, "BirthYearRange", &byr, &path);
            }
        }

        // Narrow by Age.
        if entry.age != Age::None {
            if range.narrow_by_age(entry.age, mdate) == NarrowResult::Conflict {
                trace_conflict(debug, &range, mdate, "Age", &entry.age, &path);
            }
        }

        // Narrow by AgeRange.
        if entry.agerange.min.is_some() || entry.agerange.max.is_some() {
            if range.narrow_by_range(entry.agerange.min, entry.agerange.max, mdate)
                == NarrowResult::Conflict
            {
                trace_conflict(debug, &range, mdate, "AgeRange", &entry.agerange, &path);
            }
        }

        bd_ranges.push(range);
        meet_dates.push(mdate);
    }
    let bd_groups = group_by_age(&bd_ranges, 0);

    if debug {
        print_groups(&bd_groups, &bd_ranges, &meet_dates);
    }
    //  bd_groups
}

/// Public-facing entry point for debugging a single lifter's interpolation.
pub fn group_age_debug_for(meetdata: &mut AllMeetData, liftermap: &LifterMap, username: &Username) {
    match liftermap.get(username) {
        Some(indices) => group_lifter_data(meetdata, indices, true),
        None => println!("Username '{}' not found", username),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prelim_test() {
        let birthdate1 = Date::from_parts(1965, 02, 03);
        let birthdate2 = Date::from_parts(1966, 02, 03);
        let birthdate3 = Date::from_parts(1967, 02, 03);
        let birthdate4 = Date::from_parts(1966, 02, 03);
        let birthdate5 = Date::from_parts(1936, 02, 03);

        let date1 = Date::from_parts(2020, 07, 26);
        let date2 = Date::from_parts(2017, 03, 22);
        let date3 = Date::from_parts(2005, 03, 15);

        let mut bdr1 = BirthDateRange::default();
        bdr1.narrow_by_birthdate(birthdate1);

        let mut bdr2 = BirthDateRange::default();
        bdr2.narrow_by_birthdate(birthdate2);

        let mut bdr3 = BirthDateRange::default();
        bdr3.narrow_by_birthdate(birthdate3);

        let mut bdr4 = BirthDateRange::default();
        bdr4.narrow_by_birthdate(birthdate4);

        let mut bdr5 = BirthDateRange::default();
        bdr5.narrow_by_birthdate(birthdate5);

        let mut bdr6 = BirthDateRange::default();
        bdr6.narrow_by_age(Age::Exact(84), date1);

        let mut bdr7 = BirthDateRange::default();
        bdr7.narrow_by_age(Age::Approximate(50), date2);

        let mut bdr8 = BirthDateRange::default();
        bdr8.narrow_by_range(Age::Approximate(40), Age::Approximate(99), date3);

        let age_data = [bdr1, bdr2, bdr3, bdr4, bdr5, bdr6, bdr7, bdr8];

        let grouped_data = group_by_age(&age_data, 0);

        assert_eq!(grouped_data[0], vec![4, 5, 7]);
        assert_eq!(grouped_data[1], vec![1, 3, 6]);
        assert_eq!(grouped_data[2], vec![0]);
        assert_eq!(grouped_data[3], vec![2]);
    }
}
