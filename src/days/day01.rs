use std::{fmt::Display, io, str::FromStr};

use aoc_2025::aoc_tests;

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "L"),
            Self::Right => write!(f, "R"),
        }
    }
}

#[derive(Debug)]
struct Rotation {
    direction: Direction,
    distance: i32,
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}{}", self.direction, self.distance))
    }
}

impl FromStr for Rotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, distance) = s.split_at(1);

        let direction = match dir {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(format!("Unknown direction: {dir}")),
        };

        let Ok(distance) = distance.parse::<i32>() else {
            return Err(format!("Failed to parse distance: {distance}"));
        };

        Ok(Rotation {
            direction,
            distance,
        })
    }
}

fn parse_input(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .map(|l| FromStr::from_str(l).unwrap())
        .collect()
}

struct Dial {
    pub rotation: i32,
}

impl Dial {
    fn new() -> Dial {
        Self { rotation: 50 }
    }

    fn rotate(&mut self, rot: &Rotation) {
        self.rotation = match rot.direction {
            Direction::Left => {
                let res = (self.rotation - rot.distance) % 100;
                if res >= 0 { res } else { 100 + res }
            }
            Direction::Right => (self.rotation + rot.distance) % 100,
        }
    }

    fn rotate_once(&mut self, direction: Direction) {
        self.rotate(&Rotation {
            direction,
            distance: 1,
        });
    }
}

pub fn part1(input: &str) -> Result<String, io::Error> {
    let rotations = parse_input(input);

    let mut dial = Dial::new();
    let mut points_at_zero = 0;
    for rotation in rotations {
        dial.rotate(&rotation);
        if dial.rotation == 0 {
            points_at_zero += 1;
        }
    }

    Ok(points_at_zero.to_string())
}

pub fn part2(input: &str) -> Result<String, io::Error> {
    let rotations = parse_input(input);

    let mut dial = Dial::new();
    let mut points_at_zero = 0;

    for rotation in rotations {
        for _ in 0..rotation.distance {
            dial.rotate_once(rotation.direction.clone());
            if dial.rotation == 0 {
                points_at_zero += 1;
            }
        }
    }

    Ok(points_at_zero.to_string())
}

#[allow(dead_code)]
const EXAMPLE_INPUT: &str = "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

aoc_tests!(
    EXAMPLE_INPUT,
    part1 => "3".to_string(),
    part2 => "6".to_string(),
);
