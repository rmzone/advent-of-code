use crate::{Beacon, parse_input};
use common::custom_error::Result;
use itertools::Itertools;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str, line: i64) -> Result<String> {
    let (_, sensor_data) = parse_input(input)?;
    info!("{:?}", sensor_data);
    /*
    Consider the sensor at 8,7
    This sensor's closest beacon is at 2,10, and so you know
    there are no beacons that close or closer (in any positions marked #).
    distance = 9
    range to check (7-9..7+9) => (-2..16)
    so y=10 is covered.
    the x values are (2..=14) centered on x=8 (need to reject where the actual beacon is!)
    repeat for each pair
    collect and count

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
     7 .#########S#######S#........
     8 ..#################.........
     9 ...###############..........
    10 ....B############...........
    11 ..S..###########............
    12 ......#########.............
    13 .......#######..............
    14 ........#####.S.......S.....
    15 B........###................
    16 ..........#SB...............
    17 ................S..........B
    18 ....S.......................
    19 ............................
    20 ............S......S........
    21 ............................
    22 .......................B....

        let sensor = Sensor { x: 8, y: 7 };
        let beacon = Beacon { x: 2, y: 10 };

        let distance = sensor.distance_to_beacon(&beacon);
        info!("distance: `{}`", distance);

        let y_range = sensor.y_range(distance);
        info!("y_range: {:?}", y_range);

        let is_covered = sensor.in_range(distance, line);
        info!("is_covered: `{}`", is_covered);

        let x_range = sensor.x_coverage_at_y(distance, line);
        info!("x_range: `{:?}`", x_range);
    */
    let result = sensor_data
        .iter()
        .filter_map(|(sensor, beacon)| {
            let distance = sensor.distance_to_beacon(&beacon);
            if sensor.in_range(distance, line) {
                Some(sensor.x_coverage_at_y(distance, line))
            } else {
                None
            }
        })
        .flatten()
        .unique()
        .filter(|x| !sensor_data.values().contains(&Beacon { x: *x, y: line }))
        .count();

    Ok(result.to_string())
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
        assert_eq!("26", process(input, 10)?);
        Ok(())
    }
}
