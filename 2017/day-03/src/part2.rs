use crate::parse_input;
use common::custom_error::Result;
use glam::IVec2;
use std::collections::HashMap;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, port) = parse_input(input)?;
    let mut ports: HashMap<IVec2, i32> = HashMap::new();

    ports.insert(IVec2::new(0, 0), 1);

    let result = manhattan_distance_recursive(port, 1, 2, &mut ports);

    Ok(result.to_string())
}

fn manhattan_distance_recursive(
    value: i32,
    ring: i32,
    start: i32,
    ports: &mut HashMap<IVec2, i32>,
) -> i32 {
    let circumference = ring * 8;
    let mut current = start;

    let mut dx = 0; // (0,1), (-1,0), (0, -1), (1, 0) need to flip every 6 steps
    let mut dy = 1;

    // calculate starting position
    let mut x: i32 = ring;
    let mut y: i32 = 1 - ring;

    let position = IVec2::new(x, y);
    let result = get_sum(position, &ports);

    // info!("{}", result);

    ports.insert(position, result);
    if result > value {
        return result;
    }

    // info!("Start: {} {}", x, y);

    let mut steps = 0;
    let delta = circumference / 4; // 2, 4, 6, ...

    // info!("{} => ({},{}) D({},{})", current, x, y, dx, dy);

    while current < start + circumference {
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

        // set and check if we are done
        let position = IVec2::new(x, y);
        let result = get_sum(position, &ports);

        // info!("{}", result);

        ports.insert(position, result);
        if result > value {
            return result;
        }
    }

    // If we get here, then we need to try the next ring
    manhattan_distance_recursive(value, ring + 1, start + circumference, ports)
}

const NEIGHBORS: [IVec2; 8] = [
    IVec2::new(-1, -1),
    IVec2::new(0, -1),
    IVec2::new(1, -1),
    IVec2::new(-1, 0),
    IVec2::new(1, 0),
    IVec2::new(-1, 1),
    IVec2::new(0, 1),
    IVec2::new(1, 1),
];

fn get_sum(pos: IVec2, ports: &HashMap<IVec2, i32>) -> i32 {
    let count = NEIGHBORS.iter().fold(0, |mut acc, neighbor| {
        let current = *neighbor + pos;
        if ports.contains_key(&current) {
            acc = acc + ports[&current];
        }
        acc
    });

    count
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
}
