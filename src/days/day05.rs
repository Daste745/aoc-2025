use color_eyre::eyre::Result;

use aoc_2025::aoc_tests;

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn contains(&self, n: i64) -> bool {
        self.start <= n && n <= self.end
    }

    fn size(&self) -> i64 {
        self.end - self.start
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
    let db = parse_input(input);

    // a -> 10..=100
    // b -> 80..=150
    // duplicated -> 100..=80
    //
    // HashSet -> too much results to remember

    let result = db.fresh.iter().map(Range::size).sum();

    Ok(result)
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
