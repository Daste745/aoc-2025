use std::collections::HashSet;

use color_eyre::eyre::Result;

use aoc_2025::aoc_tests;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Point2D(u32, u32);

struct Board {
    points: HashSet<Point2D>,
}

impl Board {
    fn neighbors(&self, point: &Point2D) -> impl Iterator<Item = &Point2D> {
        self.points
            .iter()
            // Up to 1 point away and not us
            .filter(|&p| *p != *point && p.0.abs_diff(point.0) <= 1 && p.1.abs_diff(point.1) <= 1)
    }
}

fn parse_input(input: &str) -> Board {
    let rolls: HashSet<Point2D> = input
        .trim()
        .split("\n")
        .enumerate()
        .flat_map(|(row, line)| {
            line.trim().chars().enumerate().filter_map(move |(col, c)| {
                if c == '@' {
                    Some(Point2D(row as u32, col as u32))
                } else {
                    None
                }
            })
        })
        .collect();

    Board { points: rolls }
}

pub fn part1(input: &str) -> Result<i64> {
    let board = parse_input(input);

    let result = board
        .points
        .iter()
        // Filter out irremovable points (>= neighbors)
        .filter(|point| board.neighbors(point).count() < 4)
        .count();

    Ok(result as i64)
}

pub fn part2(input: &str) -> Result<i64> {
    let mut board = parse_input(input);
    let original_len = board.points.len();

    loop {
        let prev_len = board.points.len();

        let new_points = board
            .points
            .iter()
            // Filter out removable points (< 4 neighbors)
            .filter(|point| board.neighbors(point).count() >= 4);
        board.points = new_points.map(|p| *p).collect();

        // No changes -> end
        if prev_len == board.points.len() {
            break;
        }
    }

    let removed = original_len - board.points.len();
    Ok(removed as i64)
}

#[allow(dead_code)]
const EXAMPLE_INPUT: &str = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

aoc_tests!(
    EXAMPLE_INPUT,
    part1 => 13,
    part2 => 43,
);
