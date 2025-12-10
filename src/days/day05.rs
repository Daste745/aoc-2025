use color_eyre::eyre::Result;

use aoc_2025::{aoc_tests, range::Range};

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
            None
        } else {
            db.fresh
                .iter()
                .enumerate()
                .skip_while(|&(i, _)| i <= idx)
                .fold(Some(*range), |acc, (_, r)| {
                    if let Some(acc) = acc {
                        acc.difference(r)
                    } else {
                        None
                    }
                })
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
