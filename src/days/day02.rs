use color_eyre::eyre::{Error, Result, eyre};
use std::str::FromStr;

use aoc_2025::aoc_tests;

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((start, end)) = s.split_once("-") else {
            return Err(eyre!("Invalid range: {s}"));
        };
        let Ok(start) = start.parse::<i64>() else {
            return Err(eyre!("Failed to parse start of range: {start}"));
        };
        let Ok(end) = end.parse::<i64>() else {
            return Err(eyre!("Failed to parse end of range: {end}"));
        };
        Ok(Self { start, end })
    }
}

fn parse_input(input: &str) -> Result<Vec<Range>> {
    input.trim().split(",").map(Range::from_str).collect()
}

pub fn part1(input: &str) -> Result<i64> {
    let ranges = parse_input(input)?;
    let ids = ranges.iter().flat_map(|r| r.start..=r.end);
    let mut invalid_id_sum: i64 = 0;

    for id in ids {
        let digits = id.ilog10() + 1;

        // Odd lengths can't be invalid
        if digits % 2 != 0 {
            continue;
        }

        let midpoint = digits / 2;
        let first = id / 10i64.pow(midpoint);
        let second = id % 10i64.pow(midpoint);
        if first == second {
            invalid_id_sum += id;
        }
    }

    Ok(invalid_id_sum)
}

pub fn part2(input: &str) -> Result<i64> {
    let ranges = parse_input(input)?;
    let ids = ranges.iter().flat_map(|r| r.start..=r.end);

    Ok(ids
        .filter(|id| {
            let digits = id.ilog10() + 1;
            let id_chars = id.to_string().chars().collect::<Vec<_>>();

            // Find all divisors of `id`
            let divisors = (1..digits).filter(|i| digits % i == 0);
            // Find if **any** combination of chunk sizes has all equal parts
            divisors.into_iter().any(|divisor| {
                let mut chunks = id_chars.chunks(divisor as usize).into_iter();
                let first = chunks.next().unwrap();
                // **All** chunks must be equal for the ID to considered be invalid
                chunks.all(|chunk| chunk == first)
            })
        })
        .sum())
}

#[allow(dead_code)]
const EXAMPLE_INPUT: &str = "
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";

aoc_tests!(
    EXAMPLE_INPUT,
    part1 => 1227775554,
    part2 => 4174379265,
);
