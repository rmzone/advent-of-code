use crate::{Beacon, DiagonalLine, Sensor, parse_input};
use common::custom_error::Result;
use itertools::Itertools;
use std::collections::HashSet;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str, limit: i64) -> Result<String> {
    let (_, sensor_data) = parse_input(input)?;
    info!("{:?}", sensor_data);

    // try brute force to make sure the problem is understood
    // trick rotate diamond so it is a square?
    // scan x,y (0,0) - (20,20)
    //   x=14, y=11

    // Identify all the diagonal gaps between sensor detection ranges that are only
    // one space wide.

    /*
                   1    1    2    2
         0    5    0    5    0    5
    -2 ..........#.................
    -1 .........###................
     0 ....S...#####...............
     1 .......#######........S.....
     2 ......#########S............
     3 .....###########SB..........
     4 ....#############...........
     5 ...###############..........
     6 ..#################.........
     7 .#########1#######S#........
     8 ..#################.........
     9 ...###############..........
    10 ....a############...........
    11 ..S..###########@...........
    12 ......#########.#...........
    13 .......#######.###..........
    14 ........#####.S####...S.....
    15 B........###.#######........
    16 ..........#Sb########.......
    17 ...........#####2#####.....B
    18 ....S.......................
    19 ............................
    20 ............S......S........
    21 ............................
    22 .......................B....

    pick a pair of sensors
    Sensor { x: 14, y: 17 } Sensor { x: 8, y: 7 }, range1: 5, range2: 9, distance: 16, total_range: 14, gap: 1
    notice the gap... make a line slope 1 x-intercept 25
    {Positive(-11), Positive(-3), Negative(25), Negative(13)}
    [Beacon { x: -12, y: 1 }, Beacon { x: -18, y: 7 }, Beacon { x: 8, y: 5 }, Beacon { x: 14, y: 11 }]
    */

    let mut diagonal_lines: HashSet<DiagonalLine> = HashSet::new();

    for [sensor1, sensor2] in sensor_data.keys().array_combinations() {
        let beacon1 = sensor_data.get(sensor1).unwrap();
        let beacon2 = sensor_data.get(sensor2).unwrap();
        let distance = (sensor1.x - sensor2.x).abs() + (sensor1.y - sensor2.y).abs();
        let range1 = sensor1.distance_to_beacon(beacon1);
        let range2 = sensor2.distance_to_beacon(beacon2);
        let total_range = range1 + range2;

        if total_range < distance {
            let gap = distance - total_range - 1;
            if gap == 1 {
                info!(
                    "{:?} {:?}, range1: {}, range2: {}, distance: {}, total_range: {}, gap: {}",
                    sensor1, sensor2, range1, range2, distance, total_range, gap
                );
                // calculate diagonal between sensor1 and sensor2 and collect
                diagonal_lines.insert(diagonal_between(sensor1, sensor2, range1, range2));
            }
        }
    }

    // Identify all the points where these one-wide gaps intersect. These are possible beacons.
    let mut possible_beacons: Vec<Beacon> = Vec::new();

    info!("{:?}", diagonal_lines);

    for [line1, line2] in diagonal_lines.iter().array_combinations() {
        if let Some(beacon) = line1.intersects(line2) {
            possible_beacons.push(beacon);
        }
    }

    info!("{:?}", possible_beacons);

    // Should only be one Beacon in range
    let mut alert: Option<Beacon> = None;
    let mut result = 0;

    'outer: for beacon in possible_beacons {
        // Is the beacon out of range
        if beacon.x < 0 || beacon.y < 0 || beacon.x > limit || beacon.y > limit {
            continue;
        }

        // see if any sensors can see this beacon
        for (sensor, b) in sensor_data.iter() {
            let range = sensor.distance_to_beacon(b);
            let distance = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();
            if range >= distance {
                continue 'outer;
            }
        }

        info!("Possible Beacon: {:?}", beacon);
        alert = Some(beacon);
        //break;
    }

    // info!("{:?}", beacons);

    if let Some(beacon) = alert {
        // Calculate tuning frequency
        result = beacon.x * 4000000 + beacon.y;
    }

    Ok(result.to_string())
}

/// Calculate the diagonal line between two sensors
fn diagonal_between(sensor1: &Sensor, sensor2: &Sensor, range1: i64, _: i64) -> DiagonalLine {
    let offset = range1 + 1; // line is just outside the range of each sensor

    // find two points on the line
    let (p1x, p1y) = if sensor1.x > sensor2.x {
        (sensor1.x - offset, sensor1.y)
    } else {
        (sensor1.x + offset, sensor1.y)
    };

    let (p2x, p2y) = if sensor1.y > sensor2.y {
        (sensor1.x, sensor1.y - offset)
    } else {
        (sensor1.x, sensor1.y + offset)
    };

    let slope = (p2x - p1x) / (p2y - p1y);
    let intercept = p1y - (slope * p1x);
    if slope > 0 {
        DiagonalLine::Positive(intercept)
    } else {
        DiagonalLine::Negative(intercept)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";
        assert_eq!("56000011", process(input, 20)?);
        Ok(())
    }
}
