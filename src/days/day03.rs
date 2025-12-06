use color_eyre::eyre::Result;

use aoc_2025::aoc_tests;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(input: &str) -> Result<i64> {
    let banks = parse_input(input);
    let mut out: i64 = 0;

    for bank in banks {
        let first_biggest = bank
            .iter()
            .enumerate()
            // Skip last value
            .filter(|(idx, _)| *idx != bank.len() - 1)
            // Keep first max instead of overriding it if duplicates are found
            .reduce(|acc, val| if val.1 > acc.1 { val } else { acc })
            .unwrap();
        let second_biggest = bank
            .iter()
            .skip(first_biggest.0 + 1)
            .enumerate()
            .reduce(|acc, val| if val.1 > acc.1 { val } else { acc })
            .unwrap();
        let jolts = first_biggest.1 * 10 + second_biggest.1;
        out += jolts as i64;
    }

    Ok(out)
}

pub fn part2(input: &str) -> Result<i64> {
    let banks = parse_input(input);
    let mut out: i64 = 0;

    for bank in banks {
        let mut remaining = 12;
        let mut already_consumed: usize = 0;

        let mut jolts: i64 = 0;
        while remaining > 0 {
            let first_biggest = bank
                .iter()
                .enumerate()
                // Skip first elements until the previously found digit
                .skip(already_consumed)
                // Ignore trailing values that are required to get a result
                .filter(|(idx, _)| *idx <= bank.len() - remaining)
                // Keep first max instead of overriding it if duplicates are found
                .reduce(|acc, val| if val.1 > acc.1 { val } else { acc })
                .unwrap();

            jolts *= 10;
            jolts += *first_biggest.1 as i64;
            already_consumed = first_biggest.0 + 1;
            remaining -= 1;
        }
        out += jolts;
    }

    Ok(out)
}

#[allow(dead_code)]
const EXAMPLE_INPUT: &str = "
987654321111111
811111111111119
234234234234278
818181911112111
";

aoc_tests!(
    EXAMPLE_INPUT,
    part1 => 357,
    part2 => 3121910778619,
);
