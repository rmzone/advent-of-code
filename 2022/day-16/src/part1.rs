use std::collections::HashMap;
use std::ops::Index;
use common::custom_error::Result;
use tracing::info;
use crate::{dijkstra, get_distance_map, parse_input, valve, Valve};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, valves) = parse_input(input)?;
    info!("{:?}", valves);

    // Map valve names to a index value based on the position in the valve list.
    let valve_map: HashMap<&str, usize> = valves.
        iter()
        .enumerate()
        .map(|(index, valve)| (valve.name, index))
        .collect();

    let distance_map = get_distance_map(&valves, &valve_map);

    info!("{:?}", distance_map);

    // Use DFS to find all possible paths, eliminating those where the valve has zero pressure
    // Calculate the released pressure for each path, return the max

    let start = valve_map["AA"];

    // collect the starting list of valves with non-zero flow rates
    let closed_valves: Vec<&Valve> = valves
        .iter()
        //.enumerate()
        .filter(|(_, valve)| valve.rate > 0)
        .map(|(index, _)| valve)
        .collect();


    Ok("total_pressure".to_string())
}


/*
pub fn process_sample(input: &str) -> Result<String> {
    let (_, mut valves) = parse_input(input)?;
    info!("{:?}", valves.len());

    // find the correct order to open the valves in order to get the maximum flow in 30min
    // this algorithm only works on the sample data
    let mut current_valve_id = "AA";
    //let mut max_rate = 0;
    let mut time_elapsed = 0;
    let mut total_pressure = 0;

    // track pressure released by each valve
    while time_elapsed < 30 {
        time_elapsed += 1;
        println!("== Minute {} ==", time_elapsed);

        // for all open valves, release pressure
        let open_valves = valves
            .iter()
            .filter(|(_, v)| v.state == ValveState::Open)
            .map(|(k, v)| *k)
            .collect::<Vec<_>>();
        if open_valves.is_empty() {
            println!("No valves are open.");
        } else {
            let pressure = open_valves.iter().fold(0, |acc, v| {
                if let Some(valve) = valves.get(v) {
                    return acc + valve.rate
                };
                acc
            });

            total_pressure += pressure;
            let list = open_valves.join(" ");
            println!("Valves {} are open, releasing {} pressure.", list, pressure);
        }

        let current_valve = valves.get(current_valve_id).unwrap();

        if current_valve.state == ValveState::Closed && current_valve.rate > 2 {
            //max_rate = current_valve.rate;
            let valve_to_open = valves.get_mut(current_valve_id).unwrap();
            valve_to_open.state = ValveState::Open;
            println!("You open valve {}.", current_valve_id);
        } else {
            if let  Some(new_valve) = get_next_open_tunnel(&current_valve, &valves) {
                current_valve_id = new_valve;
                println!("You move to valve {}.", current_valve_id);
            } else {
                println!("You can't move!!!");
            }
        }

        println!();
    }

    Ok(total_pressure.to_string())
}

fn get_next_open_tunnel<'a>(current_valve: &Valve<'a>, valves: &HashMap<&str, Valve<'a>>) -> Option<&'a str> {
    for &valve in current_valve.tunnels.iter() {
        if let Some(v) = valves.get(valve) {
            if v.state == ValveState::Closed {
                return Some(valve);
            }
        }
    }

    None
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";
        assert_eq!("1651", process(input)?);
        Ok(())
    }
}
