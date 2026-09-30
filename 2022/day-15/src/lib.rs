use std::collections::HashMap;
use std::ops::RangeInclusive;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};

pub mod part1;
pub mod part2;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Sensor {
    pub x: i64,
    pub y: i64,
}

impl Sensor {
    /// Manhattan distance between a sensor and beacon
    pub fn distance_to_beacon(&self, beacon_: &Beacon) -> i64 {
        (self.x - beacon_.x).abs() + (self.y - beacon_.y).abs()
    }

    /// Check whether a given y-index is reachable by the given sensor
    pub fn in_range(&self, distance: i64, y_index: i64) -> bool {
        let sensor_range = self.y_range(distance);
        sensor_range.contains(&y_index)
    }

    /// Returns a range of the x values, centered on the
    /// sensor's x position, that the sensor can sense
    /// at the target y-index
    pub fn x_coverage_at_y(&self, max_distance: i64, target_y_index: i64) -> RangeInclusive<i64> {
        let delta = max_distance - (self.y - target_y_index).abs();
        (self.x-delta..=self.x+delta)
    }

    /// Returns a range of the y values, representing the min-max distance from the sensor
    pub fn y_range(&self, distance: i64) -> RangeInclusive<i64> {
        (self.y - distance)..=(self.y + distance)
    }
}

#[derive(Debug)]
pub struct Beacon {
    pub x: i64,
    pub y: i64,
}

pub fn parse_input(input: &str) -> IResult<&str, HashMap<Sensor, Beacon>> {
    let (input, lines) = separated_list1(line_ending, line).parse(input)?;

    Ok((input, lines.into_iter().collect::<HashMap<Sensor, Beacon>>()))
}

fn line(input: &str) -> IResult<&str, (Sensor, Beacon)> {
    let (input, _) = tag("Sensor at ").parse(input)?;
    let (input, sensor) = map(position, |(x, y)| Sensor { x, y }).parse(input)?;
    let (input, _) = tag(": closest beacon is at ").parse(input)?;
    let (input, beacon) = map(position, |(x, y)| Beacon { x, y }).parse(input)?;

    Ok((input, (sensor, beacon)))
}

fn position(input: &str) -> IResult<&str, (i64, i64)> {
    separated_pair(
        preceded(tag("x="), nom::character::complete::i64),
        tag(", "),
        preceded(tag("y="), nom::character::complete::i64),
    ).parse(input)
}
