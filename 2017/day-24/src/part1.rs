use crate::{Port, calculate_strength, my_display, parse_input};
use common::custom_error::Result;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, ports) = parse_input(input)?;
    let mut max_strength = 0;

    build_bridge(0, &ports, &Vec::new(), &mut max_strength);

    Ok(max_strength.to_string())
}

fn build_bridge(start: i32, ports: &Vec<Port>, current_bridge: &Vec<Port>, current_max: &mut i32) {
    let current = calculate_strength(&current_bridge);
    if current > *current_max {
        *current_max = current
    }

    // find next matching possible ports
    let neighbors = ports
        .iter()
        .filter(|p| p.type1 == start || p.type2 == start)
        .collect::<Vec<&Port>>();
    for neighbor in neighbors {
        if current_bridge.contains(&neighbor) {
            // skip if the neighbor is already in the list
            continue;
        }

        let mut next = current_bridge.clone();
        next.push(neighbor.clone());

        if neighbor.type1 == start {
            build_bridge(neighbor.type2, &ports, &next, current_max);
        } else {
            build_bridge(neighbor.type1, &ports, &next, current_max);
        }
    }

    info!("{}", my_display(current_bridge));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
