use std::fmt::Display;

use color_eyre::eyre::Result;

use aoc_2025::aoc_tests;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    #[allow(dead_code)]
    fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }

    fn contains(&self, n: i64) -> bool {
        self.start <= n && n <= self.end
    }

    fn size(&self) -> i64 {
        self.end - self.start + 1
    }

    fn fully_contains(&self, other: &Range) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    /// Find the difference between self and another Range.
    ///
    /// If the difference contains zero entries, return None.
    ///
    /// If the difference contains at least one entry, return a new Range.
    fn difference(&self, other: &Range) -> Option<Range> {
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
mod range_tests {
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

struct Database {
    fresh: Vec<Range>,
    available: Vec<i64>,
}

fn parse_input(input: &str) -> Database {
    let mut parts = input.trim().split("\n\n");

    let fresh = parts
        .next()
        .expect("Fresh ingredient ID ranges")
        .split("\n")
        .map(|line| {
            let (start, end) = line.trim().split_once("-").expect("ID range");
            Range {
                start: start.parse::<i64>().unwrap(),
                end: end.parse::<i64>().unwrap(),
            }
        })
        .collect();

    let available = parts
        .next()
        .expect("Available ingredient IDs")
        .split("\n")
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    Database { fresh, available }
}

pub fn part1(input: &str) -> Result<i64> {
    let db = parse_input(input);

    let result = db
        .available
        .iter()
        .filter(|&&id| db.fresh.iter().any(|range| range.contains(id)))
        .count();

    Ok(result as i64)
}

pub fn part2(input: &str) -> Result<i64> {
    let mut db = parse_input(input);

    db.fresh.sort_by(|a, b| a.start.cmp(&b.start));
    let ranges = db.fresh.iter().enumerate().filter_map(|(idx, range)| {
        if db
            .fresh
            .iter()
            .filter(|&other_range| other_range != range)
            .any(|other_range| other_range.fully_contains(range))
        {
            println!("DROP\t{range}");
            None
        } else {
            let combined = db
                .fresh
                .iter()
                .enumerate()
                .skip_while(|&(i, _)| i <= idx)
                // .filter(|&(_, r)| r != range)
                .fold(Some(*range), |acc, (_, r)| {
                    if let Some(acc) = acc {
                        let combined = acc.difference(r);
                        println!("  DIFF\t{acc} - {r}\t-> {combined:?}");
                        combined
                    } else {
                        println!("  DIFF -> None");
                        None
                    }
                });
            println!("COMBINE\t{range} -> {combined:?}");
            combined
        }
    });

    Ok(ranges.map(|range| range.size()).sum())
}

#[allow(dead_code)]
const EXAMPLE_INPUT: &str = "
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

aoc_tests!(
    EXAMPLE_INPUT,
    part1 => 3,
    part2 => 14,
);
