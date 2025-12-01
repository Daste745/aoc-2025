use std::io;

pub fn part1(input: &str) -> Result<String, io::Error> {
    let mut cnt = 0;
    for line in input.lines() {
        cnt += 1;
        println!("{line}")
    }

    Ok(cnt.to_string())
}

pub fn part2(input: &str) -> Result<String, io::Error> {
    let mut cnt = 0;
    for line in input.lines() {
        cnt += 1;
        println!("{line}")
    }

    Ok(cnt.to_string())
}
