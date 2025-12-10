use std::fmt::Display;

/// Inclusive range
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Range {
    pub start: i64,
    pub end: i64,
}

impl Range {
    pub fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }

    pub fn contains(&self, n: i64) -> bool {
        self.start <= n && n <= self.end
    }

    pub fn size(&self) -> i64 {
        self.end - self.start + 1
    }

    /// Check if a `Range` fully contains another `Range`.
    pub fn fully_contains(&self, other: &Range) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    /// Find the difference between self and another `Range`.
    ///
    /// If the difference contains zero entries, return None.
    ///
    /// If the difference contains at least one entry, return a new `Range`.
    pub fn difference(&self, other: &Range) -> Option<Range> {
        if other.fully_contains(self) {
            return None;
        }

        // Trim to be after the end of `other` if necessary
        let start = if self.start > other.start && self.start <= other.end {
            other.end + 1
        } else {
            self.start
        };

        // Trim to be before the start of `other` if necessary
        let end = if self.end < other.end && self.end >= other.start {
            other.start - 1
        } else {
            self.end
        };

        Some(Range { start, end })
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}..={})", self.start, self.end))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// (3..=7) - (8..=13) = (3..=7) (unchanged)
    #[test]
    fn difference_with_separate_end_and_start() {
        let a = Range::new(3, 7);
        let b = Range::new(8, 13);
        assert_eq!(a.difference(&b), Some(Range::new(3, 7)));
    }

    /// (3..=7) - (7..=13) = (3..=6) (trimmed end)
    #[test]
    fn difference_with_overlapping_end_and_start() {
        let a = Range::new(3, 7);
        let b = Range::new(7, 13);
        assert_eq!(a.difference(&b), Some(Range::new(3, 6)));
    }

    /// (7..=10) - (3..=6) = (7..=10) (unchanged)
    #[test]
    fn difference_with_separate_start_and_end() {
        let a = Range::new(7, 10);
        let b = Range::new(3, 6);
        assert_eq!(a.difference(&b), Some(Range::new(7, 10)));
    }

    /// (7..=10) - (3..=7) = (8..=10) (trimmed start)
    #[test]
    fn difference_with_overlapping_start_and_end() {
        let a = Range::new(7, 10);
        let b = Range::new(3, 7);
        assert_eq!(a.difference(&b), Some(Range::new(8, 10)));
    }

    /// (3..=7) - (7..=13) = None (fully contained in 2nd range)
    #[test]
    fn difference_with_full_overlap() {
        let a = Range::new(3, 7);
        let b = Range::new(3, 7);
        assert_eq!(a.difference(&b), None);
    }
}
