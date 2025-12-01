use std::{fmt::Display, io, ops::Range, str::FromStr};

#[derive(Debug)]
enum Rotation {
    Left { distance: i32 },
    Right { distance: i32 },
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left { distance } => f.write_fmt(format_args!("L{distance}")),
            Self::Right { distance } => f.write_fmt(format_args!("R{distance}")),
        }
    }
}

impl FromStr for Rotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, distance) = s.split_at(1);
        let Ok(distance) = distance.parse::<i32>() else {
            return Err(format!("Failed to parse distance: {distance}"));
        };

        match dir {
            "L" => Ok(Self::Left { distance }),
            "R" => Ok(Self::Right { distance }),
            _ => Err(format!("Unknown direction: {dir}")),
        }
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
        self.rotation = match rot {
            Rotation::Left { distance } => {
                let res = (self.rotation - distance) % 100;
                if res >= 0 { res } else { 100 + res }
            }
            Rotation::Right { distance } => (self.rotation + distance) % 100,
        }
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
        let amount = match rotation {
            Rotation::Left { distance } => distance,
            Rotation::Right { distance } => distance,
        };
        for _ in 0..amount {
            dial.rotate(&match rotation {
                Rotation::Left { distance: _ } => Rotation::Left { distance: 1 },
                Rotation::Right { distance: _ } => Rotation::Right { distance: 1 },
            });
            if dial.rotation == 0 {
                points_at_zero += 1;
            }
        }
    }

    Ok(points_at_zero.to_string())
}
