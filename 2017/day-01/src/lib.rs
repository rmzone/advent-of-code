pub mod part1;
pub mod part2;

pub fn parse_input(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect()
}
