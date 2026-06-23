use crate::{find_bottom, parse_input, Tower};
use common::custom_error::Result;
use itertools::Itertools;
use std::collections::HashMap;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (_, mut tower_map) = parse_input(input)?;
    let bottom = find_bottom(&mut tower_map);
    calculate_weights(&mut tower_map, bottom);
    let result = balance_towers(&tower_map, bottom, 0);
    Ok(result.to_string())
}

/*
    tknk (41) -> ugml, padx, fwft
        ugml (68) -> gyxo, ebii, jptl
           gyxo (61)
           ebii (61)
           jptl (61)
        padx (45) -> pbga, havc, qoyq
            ...
        fwft (72) -> ktlj, cntj, xhth
            ...
*/
fn balance_towers(tower_map: &HashMap<&str, Tower>, tower_name: &str, expected: i32) -> i32 {
    let tower = tower_map.get(tower_name).unwrap();

    if let Some(discs) = &tower.discs {
        let children = discs
            .iter()
            .map(|&disc| tower_map.get(disc).unwrap().total_weight)
            .collect::<Vec<_>>();
        info!("Child weights: {} {:?}", &tower_name, &children);

        if let Some(i) = is_odd_man_out(&children) {
            let odd_tower = discs[i];
            // find the other either index 0 or 1
            let other = if i == 0 { 1 } else { 0 };
            return balance_towers(&tower_map, odd_tower, children[other]);
        } else {
            // I'm the one?
            // 68 - (251 - 243) = 60
            return tower.weight - (tower.total_weight - expected);
        }
    }

    tower.total_weight
}

/// Find the odd man out. From the problem there will only be one.
/// Also, there are no places where there is only one child. # children == 0 or > 1.
fn is_odd_man_out(children: &Vec<i32>) -> Option<usize> {
    for (index, &weight) in children.iter().enumerate() {
        if children.iter().filter(|&w| w == &weight).count() == 1 {
            // what if there are only 2 children?
            return Some(index);
        }
    }

    None
}

fn calculate_weights(tower_map: &mut HashMap<&str, Tower>, tower_name: &str) -> i32 {
    let (weight, discs) = {
        let tower = tower_map.get(tower_name).unwrap();
        (tower.weight, tower.discs.clone())
    };

    let mut total_weight = weight;

    if let Some(discs) = discs {
        for disk in discs {
            total_weight += calculate_weights(tower_map, disk);
        }
    }

    let tower = tower_map.get_mut(tower_name).unwrap();
    tower.total_weight = total_weight;

    tower.total_weight
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
        assert_eq!("60", process(input)?);
        Ok(())
    }

    #[test]
    fn test_is_odd_man_out() {
        assert_eq!(None, is_odd_man_out(&vec![61, 61, 61]));

        assert_eq!(Some(1), is_odd_man_out(&vec![61, 42, 61]));

        assert_eq!(Some(0), is_odd_man_out(&vec![61, 42]));
    }
}
