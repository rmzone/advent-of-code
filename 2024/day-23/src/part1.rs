use crate::{build_connection_map, parse_input};
use common::custom_error::{Error, Result};
use std::collections::{HashMap, HashSet, VecDeque};

pub fn process(input: &str) -> Result<String, Error> {
    let (_, items) = parse_input(input).expect("Error parsing input");
    let connections = build_connection_map(items);

    let mut connected = HashSet::new();
    // let mut visited: HashSet<&str> = HashSet::new();

    for &name in connections.keys() {
        // let name = "co";
        let all = find_connected(name, 3, &connections /*, &mut visited*/);

        for item in all {
            // println!("{} -> {}", name, item);
            connected.insert(item);
        }
    }

    // println!("{:?}", &result);
    let mut result = 0;

    for c in connected.iter() {
        let s = c.split(",").collect::<Vec<&str>>();
        let count = s.iter().filter(|&&x| x.starts_with("t")).count();
        if count > 0 {
            // println!("{}", c);
            result += 1;
        }
    }

    Ok(result.to_string()) // 2456 is too high 1314
}

/*
    Example ('aq', 3) => (aq,cg,yn) (aq,vc,wq)

        $ means already visited

        'aq' -> 'yn', 'vc', 'cg', 'wq'                depth:3
            'yn' -> '$aq', 'cg', 'wh', 'td'             depth:2
                'cg' -> 'de', 'tb', '$yn', '$aq'          depth:1*
                'wh' -> 'tc', 'td', '$yn', 'qp'           depth:1
                'td' -> 'tc', 'wh', '$yn', 'qp'           depth:1
            'vc' -> '$aq', 'ub', 'wq', 'tb'             depth:2
                'ub' -> 'qp' 'kh', 'wq', '$vc'            depth:1
                'wq' -> 'tb', 'ub', '$aq', '$vc'           depth:1*
                'tb' -> 'cg', 'ka', 'wq', '$vc'           depth:1
            'cg' -> 'de', 'tb', 'yn', '$aq'             depth:2
                'de' -> '$cg', 'co', 'ta', 'ka'            depth:1
                'tb' -> '$cg', 'ka', 'wq', 'vc'           depth:1
                'yn' -> '$aq', '$cg', 'wh', 'td'          depth:1*
            'wq' -> 'tb', 'ub', '$aq', 'vc'             depth:2
                'tb' -> 'cg', 'ka', '$wq', 'vc'          depth:1
                'ub' -> 'qp', 'kh', '$wq', 'vc'          depth:1
                'vc' -> '$aq', 'ub', '$wq', 'tb'          depth:1*

          4 results but only 2 unique (maybe sort alphabetically?) but not in this method
          use BFS algorithm
*/
fn find_connected<'a>(
    name: &'a str,
    depth: i32,
    connections: &HashMap<&str, HashSet<&'a str>>, /*, visited: &mut HashSet<&'a str>*/
) -> Vec<String> {
    let mut queue = VecDeque::new();
    queue.push_back((name, depth, vec![name] /*, HashSet::new()*/));
    let mut paths = Vec::new();
    let mut visited: HashSet<&str> = HashSet::new();

    while let Some((node, depth, mut path /*, mut visited*/)) = queue.pop_front() {
        // If we found the end, add the path to the list of paths if it's part of the lowest.
        if depth == 1
            && connections.contains_key(&node)
            && connections.get(&node).unwrap().contains(&name)
        {
            path.sort();
            paths.push(path.join(","));
            continue;
        }

        // Check to see if we have already visited this node. If we are
        // continuing, add it to our visited set.
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);

        // For each neighbor create a new path that includes it and add it to out queue.
        for &next in connections.get(&node).unwrap() {
            if !visited.contains(&next) {
                let mut path: Vec<&str> = path.clone();
                path.push(next);
                queue.push_back((next, depth - 1, path.clone() /*, visited.clone()*/));
                // visited.insert(next);
            }
        }
    }

    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        assert_eq!("7", process(input)?);
        Ok(())
    }
}
