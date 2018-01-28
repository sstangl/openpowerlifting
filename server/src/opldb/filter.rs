//! Definition and testing of filters for `OplDb` data.

/// A list of indices to vector items that all have the same property.
///
/// Accessing the vectors through indices allows effective
/// creation of sublists. Union and Intersection operations allow
/// for simple and efficient construction of complex filters
/// with easy caching.
///
/// The list of indices must be monotonically increasing.
#[derive(Debug,PartialEq)]
pub struct Filter {
    /// A list of monotonically increasing indices.
    pub list: Vec<u32>,
}

impl Filter {
    /// Constructs a new `Filter` that contains the indices
    /// from both source filters.
    pub fn union(&self, other: &Filter) -> Filter {
        debug_assert!(self.maintains_invariants());
        debug_assert!(other.maintains_invariants());

        // March and add the least element to the list.
        let mut acc = Vec::<u32>::with_capacity(self.list.len().max(other.list.len()));

        let mut self_index = 0;
        let mut other_index = 0;

        while self_index < self.list.len() && other_index < other.list.len() {
            let a = self.list[self_index];
            let b = other.list[other_index];

            if a == b {
                acc.push(a);
                self_index += 1;
                other_index += 1;
            } else if a < b {
                acc.push(a);
                self_index += 1;
            } else {
                acc.push(b);
                other_index += 1;
            }
        }

        // One of the lists is depleted.
        // Accumulate what remains of the other list.
        for &n in self.list.iter().skip(self_index) {
            acc.push(n);
        }
        for &n in other.list.iter().skip(other_index) {
            acc.push(n);
        }

        Filter { list: acc }
    }

    /// Constructs a new `Filter` that contains only the indices
    /// that occur in both source filters.
    pub fn intersect(&self, other: &Filter) -> Filter {
        debug_assert!(self.maintains_invariants());
        debug_assert!(other.maintains_invariants());

        // March and matching elements to the list.
        let mut acc = Vec::<u32>::new();

        let mut self_index = 0;
        let mut other_index = 0;

        while self_index < self.list.len() && other_index < other.list.len() {
            let a = self.list[self_index];
            let b = other.list[other_index];

            if a == b {
                acc.push(a);
                self_index += 1;
                other_index += 1;
            } else if a < b {
                self_index += 1;
            } else {
                other_index += 1;
            }
        }

        Filter { list: acc }
    }

    /// Tests that the list is monotonically increasing.
    #[cfg(debug_assertions)]
    pub fn maintains_invariants(&self) -> bool {
        if self.list.len() == 0 {
            return true;
        }

        let mut prev = self.list[0];
        for &i in self.list.iter().skip(1) {
            if prev >= i {
                return false;
            }
            prev = i;
        }
        return true;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_detect_nonmonotonic() {
        let f = Filter { list: vec![1,2,3,5,4] };
        assert!(!f.maintains_invariants());
    }

    #[test]
    fn test_union_basic() {
        let f1 = Filter { list: vec![1,2,3] };
        assert_eq!(f1.union(&f1), f1);

        let f1 = Filter { list: vec![0,2,6] };
        let f2 = Filter { list: vec![1,2,7] };
        let expected = Filter { list: vec![0,1,2,6,7] };
        assert_eq!(f1.union(&f2), expected);
        assert_eq!(f2.union(&f1), expected);
    }

    #[test]
    fn test_union_empty() {
        let empty = Filter { list: vec![] };
        assert_eq!(empty.union(&empty), empty);

        let f2 = Filter { list: vec![1,2,3] };
        assert_eq!(empty.union(&f2), f2);
        assert_eq!(f2.union(&empty), f2);
    }

    #[test]
    fn test_intersect_basic() {
        let f1 = Filter { list: vec![1,2,3] };
        assert_eq!(f1.intersect(&f1), f1);

        let f1 = Filter { list: vec![0,2,4,6,8] };
        let f2 = Filter { list: vec![0,3,4,8,10,12] };
        let expected = Filter { list: vec![0,4,8] };
        assert_eq!(f1.intersect(&f2), expected);
        assert_eq!(f2.intersect(&f1), expected);
    }

    #[test]
    fn test_intersect_empty() {
        let empty = Filter { list: vec![] };
        assert_eq!(empty.intersect(&empty), empty);

        let f2 = Filter { list: vec![1,2,3] };
        assert_eq!(empty.intersect(&f2), empty);
        assert_eq!(f2.intersect(&empty), empty);
    }
}
