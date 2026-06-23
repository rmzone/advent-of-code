use crate::parse_input;
use common::custom_error::Result;
use log::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, port) = parse_input(input)?;

    if port <= 1 {
        return Ok("0".to_string());
    }

    // first find ring
    let mut ring = 0;
    let mut start = 1;

    while true {
        let current_len = 8 * ring;

        // let mut end = start + current_len;
        // if end != 1 { end -= 1}
        // info!("ring: {}, start: {}, end: {}", &ring, &start, end);

        if start <= port && port < start + current_len {
            // we found the correct ring
            // info!("FOUND: ring: {}, start: {}", &ring, &start);
            break;
        }

        // next ring
        start += current_len + if current_len == 0 { 1 } else { 0 };
        ring += 1;
    }

    info!(
        "ring: {}, start: {}, end: {}",
        &ring,
        &start,
        start + ring * 8 - 1
    );
    let distance = manhattan_distance(port, ring, start);

    Ok(distance.to_string())
}

/*
17  16  15  14  13
18   5   4   3  12
19   6   1   2  11
20   7   8   9  10
21  22  23  24  25 ---> ...

            61
37  36  35  34  33  32  31
38  17  16  15  14  13  30
39  18   5   4   3  12  29
40  19   6   1   2  11  28
41  20   7   8   9  10  27
42  21  22  23  24  25  26
43  44  45  46  47  48  49  50

https://en.wikipedia.org/wiki/Taxicab_geometry

size of each range:
1  [1..=1],
8  [2..=9],
16 [10..=25],
24 [26..=49],
?? [50..=]

// how do we get the (x,y) if we have the right spiral

37  36  35  34  33  32  31
38                      30
39                      29
40           1          28
41                      27
42                      26*
43  44  45  46  47  48  49

dx,dy = 3 == ring

26 (x,y) = (3, -2) relative to 1 (BF: march around ring)

ex:  value 12, ring 2, start 10
-2  -1   0   1   2
----------------------
17  16  15  14  13 | 2
18   5   4   3  12 | 1
19   6   1   2  11 | 0
20   7   8   9  10*| -1
21  22  23  24  25 | -2

5   4   3
6   1   2*
7   8   9

*/
fn manhattan_distance(value: i32, ring: i32, start: i32) -> i32 {
    if ring == 0 {
        return 0;
    }

    let circumference = ring * 8;
    let mut current = start;
    let mut dx = 0; // (0,1), (-1,0), (0, -1), (1, 0) need to flip every 6 steps
    let mut dy = 1;

    // calculate starting position
    let mut x: i32 = ring;
    let mut y: i32 = 1 - ring;

    info!("Start: {} {}", x, y);

    let mut steps = 0;
    let delta = circumference / 4; // 2, 4, 6, ...

    // info!("{} => ({},{}) D({},{})", current, x, y, dx, dy);

    while current < value {
        x += dx;
        y += dy;
        steps += 1;
        current += 1;

        // info!("{} => ({},{}) D({},{})", current, x, y, dx, dy);

        if steps == delta - 1 {
            // 1, 3, 5, ...
            dx = -1;
            dy = 0;
        } else if steps == delta * 2 - 1 {
            // 2, 7, 11, ...
            dx = 0;
            dy = -1;
        } else if steps == delta * 3 - 1 {
            // 5, 11, 17, ...
            dx = 1;
            dy = 0;
        }
    }

    info!("Result: {} {}", x, y);

    x.abs() + y.abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "1024";
        assert_eq!("31", process(input)?);
        Ok(())
    }

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(0, manhattan_distance(1, 0, 1));
        assert_eq!(3, manhattan_distance(12, 2, 10));
        assert_eq!(2, manhattan_distance(23, 2, 10));
        assert_eq!(31, manhattan_distance(1024, 16, 962));
        assert_eq!(475, manhattan_distance(277678, 263, 275626));
    }
}
