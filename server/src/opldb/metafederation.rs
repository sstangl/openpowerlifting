//! MetaFederation definitions and calculations.

use itertools::Itertools;
use opltypes::*;
use strum::IntoEnumIterator;

use opldb::{Entry, Meet};

/// Enum of MetaFederations. These are the entries in the federation selector
/// that don't correspond neatly to just a single federation value.
///
/// Definition of each MetaFederation is in the `contains` function.
#[derive(Copy, Clone, Debug, Deserialize, Display, PartialEq, Serialize, EnumIter,
         EnumString)]
pub enum MetaFederation {
    /// Federations that are exclusively (non-optionally) tested.
    #[strum(to_string = "all-tested")]
    AllTested,
    /// All entries that have "Tested = Yes".
    #[strum(to_string = "all-amateur")]
    AllAmateur,
    #[strum(to_string = "all-argentina")]
    AllArgentina,
    #[strum(to_string = "all-australia")]
    AllAustralia,
    #[strum(to_string = "all-canada")]
    AllCanada,
    #[strum(to_string = "all-croatia")]
    AllCroatia,
    #[strum(to_string = "all-czechia")]
    AllCzechia,
    #[strum(to_string = "all-finland")]
    AllFinland,
    #[strum(to_string = "all-germany")]
    AllGermany,
    #[strum(to_string = "all-ireland")]
    AllIreland,
    #[strum(to_string = "all-israel")]
    AllIsrael,
    #[strum(to_string = "all-newzealand")]
    AllNewZealand,
    #[strum(to_string = "all-russia")]
    AllRussia,
    #[strum(to_string = "all-uk")]
    AllUK,
    #[strum(to_string = "all-ukraine")]
    AllUkraine,
    #[strum(to_string = "all-usa")]
    AllUSA,

    #[strum(to_string = "aapf")]
    AAPF,
    #[strum(to_string = "abpu")]
    ABPU,
    /// The BP federation is made up of smaller divisional federations,
    /// but people expect to see them all lumped together.
    #[strum(to_string = "all-bp")]
    AllBP,
    /// IPA, but only counting meets held in Canada.
    #[strum(to_string = "ipa-can")]
    IPACAN,
    #[strum(to_string = "uspa-tested")]
    USPATested,
}

impl MetaFederation {
    /// Defines whether a given `Entry` is part of the MetaFederation.
    ///
    /// Matching is done on Entries instead of on Meets since a MetaFederation
    /// can include Entry-specific information such as Tested status.
    pub fn contains(self, entry: &Entry, meets: &[Meet]) -> bool {
        let meet: &Meet = &meets[entry.meet_id as usize];

        match self {
            MetaFederation::AllTested => {
                // Still check entry.tested: some fully-tested federations
                // existed before drug-testing was available.
                entry.tested && meet.federation.is_fully_tested()
            },
            MetaFederation::AllAmateur => entry.tested,
            MetaFederation::AllArgentina => {
                entry.lifter_country == Some(Country::Argentina)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Argentina))
            }
            MetaFederation::AllAustralia => {
                entry.lifter_country == Some(Country::Australia)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Australia))
            }
            MetaFederation::AllCanada => {
                entry.lifter_country == Some(Country::Canada)
                    || (entry.lifter_country == None
                        && (meet.federation.home_country() == Some(Country::Canada)
                            || MetaFederation::IPACAN.contains(entry, meets)))
            }
            MetaFederation::AllCroatia => {
                entry.lifter_country == Some(Country::Croatia)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Croatia))
            }
            MetaFederation::AllCzechia => {
                entry.lifter_country == Some(Country::Czechia)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Czechia))
            }
            MetaFederation::AllFinland => {
                entry.lifter_country == Some(Country::Finland)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Finland))
            }
            MetaFederation::AllGermany => {
                entry.lifter_country == Some(Country::Germany)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Germany))
            }
            MetaFederation::AllIreland => {
                entry.lifter_country == Some(Country::Ireland)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Ireland))
            }
            MetaFederation::AllIsrael => {
                entry.lifter_country == Some(Country::Israel)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Israel))
            }
            MetaFederation::AllNewZealand => {
                entry.lifter_country == Some(Country::NewZealand)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::NewZealand))
            }
            MetaFederation::AllRussia => {
                entry.lifter_country == Some(Country::Russia)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Russia))
            }
            MetaFederation::AllUK => {
                entry.lifter_country.map_or(false, |c| c.is_in_uk())
                    || (entry.lifter_country == None
                        && meet.federation.home_country().map_or(false, |c| c.is_in_uk()))
            }
            MetaFederation::AllUkraine => {
                entry.lifter_country == Some(Country::Ukraine)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Ukraine))
            }
            MetaFederation::AllUSA => {
                entry.lifter_country == Some(Country::USA)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::USA))
            }
            MetaFederation::AAPF => meet.federation == Federation::APF && entry.tested,
            MetaFederation::ABPU => meet.federation == Federation::BPU && entry.tested,
            MetaFederation::AllBP => {
                meet.federation == Federation::BAWLA
                    || meet.federation == Federation::BP
                    || meet.federation == Federation::EPA
                    || meet.federation == Federation::NIPF
                    || meet.federation == Federation::ScottishPL
                    || meet.federation == Federation::WelshPA
            }
            MetaFederation::IPACAN => {
                meet.federation == Federation::IPA && meet.country == Country::Canada
            }
            MetaFederation::USPATested => {
                meet.federation == Federation::USPA && entry.tested
            }
        }
    }
}

/// Pre-computed list of meets in a MetaFederation.
///
/// A meet is part of the MetaFederation if it contains
/// at least one entry such that `MetaFederation::contains(entry)`.
pub struct MetaFederationCache {
    /// Uses (MetaFederation as usize) as index to a list of meet_ids
    /// for that MetaFederation.
    cache: Vec<Vec<u32>>,
}

impl MetaFederationCache {
    pub fn get_meet_ids_for<'a>(&'a self, meta: MetaFederation) -> &'a Vec<u32> {
        &self.cache[meta as usize]
    }

    /// Fill in the MetaFederationCache during CSV importation.
    ///
    /// The `entries` vector should be sorted by `entry.meet_id`,
    /// not by `entry.lifter_id` as it is post-importation.
    pub fn make(meets: &[Meet], entries: &[Entry]) -> MetaFederationCache {
        let num_metafeds: usize = MetaFederation::iter().count();

        // Vector of list of meets for each MetaFederation.
        let mut ret: Vec<Vec<u32>> = Vec::with_capacity(num_metafeds);
        for _ in 0..num_metafeds {
            ret.push(vec![]);
        }

        // Vector of whether each meet has a match for the
        // given MetaFederation (accessed via index).
        let mut contains: Vec<bool> = Vec::with_capacity(num_metafeds);
        for _ in 0..num_metafeds {
            contains.push(false);
        }

        let mut last_meet_id = 0;

        // Iterate by grouping entries from the same Meet.
        for (meet_id, meet_entries) in entries.iter().group_by(|e| e.meet_id).into_iter()
        {
            // Sanity checking that the entries argument is sorted by meet_id.
            assert!(last_meet_id <= meet_id);
            last_meet_id = meet_id;

            // Check whether any entries are part of each MetaFederation.
            for entry in meet_entries {
                for meta in MetaFederation::iter() {
                    if meta.contains(&entry, &meets) {
                        contains[meta as usize] = true;
                    }
                }
            }

            // If any match, add to that MetaFederation's meet list.
            for i in 0..num_metafeds {
                if contains[i] {
                    ret[i].push(meet_id);
                }
                // Reset the vector for the next iteration.
                contains[i] = false;
            }
        }

        // Since we're here already, sort the return vector by Date
        // in reverse order -- that's usually how it's consumed.
        for v in &mut ret {
            v.sort_unstable_by(|&a, &b| {
                meets[a as usize]
                    .date
                    .cmp(&meets[b as usize].date)
                    .reverse()
            });
        }

        MetaFederationCache { cache: ret }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    /// The Federation and MetaFederation enums must serialize
    /// to unique strings.
    #[test]
    fn test_fed_metafed_conflicts() {
        let v1: Vec<String> = Federation::iter().map(|f| f.to_string()).collect();
        let v2: Vec<String> = MetaFederation::iter().map(|f| f.to_string()).collect();
        for v in v1 {
            assert!(!v2.contains(&v));
        }
    }
}

