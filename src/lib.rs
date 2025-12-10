pub mod range;

#[macro_export]
macro_rules! aoc_tests {
    ($input:ident, $part1:ident => $part1_example:expr, $part2:ident => $part2_example:expr,) => {
        #[cfg(test)]
        mod tests {
            #[test]
            fn part1_example() {
                assert_eq!(super::$part1(super::$input.trim()).unwrap(), $part1_example);
            }

            #[test]
            fn part2_example() {
                assert_eq!(super::$part2(super::$input.trim()).unwrap(), $part2_example);
            }
        }
    };
}
