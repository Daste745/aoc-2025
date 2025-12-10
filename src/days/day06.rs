use std::collections::HashMap;

use color_eyre::eyre::Result;

use aoc_2025::aoc_tests;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mul,
}

#[derive(Debug, Clone)]
struct Problem {
    numbers: Vec<i64>,
    operator: Operator,
}

fn parse_input(input: &str) -> Vec<Problem> {
    let mut problems: HashMap<usize, Problem> = HashMap::new();

    let mut lines = input.trim().lines().peekable();
    while let Some(line) = lines.next() {
        for (idx, part) in line.split_ascii_whitespace().enumerate() {
            let problem = problems.entry(idx);
            if lines.peek().is_none() {
                let op = match part {
                    "+" => Operator::Add,
                    "*" => Operator::Mul,
                    _ => panic!("Unknown operator: {part}"),
                };
                problem
                    .and_modify(|problem| problem.operator = op)
                    .or_insert(Problem {
                        numbers: vec![],
                        operator: op,
                    });
            } else {
                let num = part.parse::<i64>().unwrap();
                problem
                    .and_modify(|problem| problem.numbers.push(num))
                    .or_insert(Problem {
                        numbers: vec![num],
                        operator: Operator::Add,
                    });
            }
        }
    }

    problems.values().cloned().collect()
}

pub fn part1(input: &str) -> Result<i64> {
    let problems = parse_input(input);

    let result = problems
        .iter()
        .map(|problem| match problem.operator {
            Operator::Add => problem.numbers.iter().sum::<i64>(),
            Operator::Mul => problem.numbers.iter().product(),
        })
        .sum();

    Ok(result)
}

pub fn part2(input: &str) -> Result<i64> {
    parse_input(input);

    Ok(0)
}

#[allow(dead_code)]
const EXAMPLE_INPUT: &str = "
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

aoc_tests!(
    EXAMPLE_INPUT,
    part1 => 4277556,
    part2 => 3263827,
);
