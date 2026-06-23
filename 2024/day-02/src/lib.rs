pub mod part1;
pub mod part2;

use nom::character::complete;
use nom::character::complete::{line_ending, space1};
use nom::multi::separated_list1;
use nom::IResult;

pub type Report = Vec<i32>;

pub fn parse_input(input: &str) -> IResult<&str, Vec<Report>> {
    separated_list1(line_ending, parse_line)(input)
}

pub fn check_safe(report: &Report) -> bool {
    let mut increasing_count = 0;
    let mut decreasing_count = 0;
    let mut min_diff = 99;
    let mut max_diff = -1;

    for i in 0..report.len() - 1 {
        let a = report[i];
        let b = report[i + 1];

        if b > a {
            increasing_count += 1;
        }

        if a > b {
            decreasing_count += 1;
        }

        let diff = (a - b).abs();

        if min_diff > diff {
            min_diff = diff;
        }

        if max_diff < diff {
            max_diff = diff;
        }
    }

    (increasing_count == report.len() - 1 || decreasing_count == report.len() - 1)
        && min_diff >= 1
        && max_diff <= 3
}

pub fn check_safe_loop(report: &Report) -> bool {
    // base case
    if check_safe(report) {
        return true;
    }

    // loop through all combinations where removing none and then each item
    for i in 0..report.len() {
        let mut temp = Vec::new();
        for x in 0..report.len() {
            if x != i {
                temp.push(report[x])
            }
        }

        if check_safe(&temp) {
            return true;
        }
    }

    false
}

fn parse_line(input: &str) -> IResult<&str, Report> {
    separated_list1(space1, complete::i32)(input)
}
