use crate::{build_connection_map2, parse_input};
use common::custom_error::{Error, Result};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn process(input: &str) -> Result<String, Error> {
    let (_, items) = parse_input(input).expect("Error parsing input");
    let (mut vertices, edges) = build_connection_map2(items);

    let clique = bron_kerbosch(&HashSet::new(), &mut vertices, &mut HashSet::new(), &edges);
    let mut largest_clique = clique.iter().collect::<Vec<_>>();
    largest_clique.sort();
    let result = largest_clique.iter().join(",");

    Ok(result)
}

/*
   find the deepest chain of comupters and build a set DFS
   Bron–Kerbosch algorithm?
*/

// Bron–Kerbosch algorithm is an enumeration algorithm for finding all maximal cliques in an undirected graph.
// https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
// r: current clique
// p: candidate vertices
// x: excluded vertices
// edges: map of connections
pub fn bron_kerbosch<'a>(
    r: &HashSet<&'a str>,
    p: &mut HashSet<&'a str>,
    x: &mut HashSet<&'a str>,
    edges: &HashMap<&'a str, HashSet<&'a str>>,
) -> HashSet<&'a str> {
    if p.is_empty() && x.is_empty() {
        // we found a maximal clique
        return r.clone();
    }

    let mut maximum_clique = HashSet::new();

    for vertex in p.clone() {
        // build new r, p, x
        let mut r = r.clone();
        r.insert(vertex);

        let neighbors = edges.get(vertex).unwrap();
        let mut new_p = p.intersection(neighbors).cloned().collect();
        let mut new_x = x.intersection(neighbors).cloned().collect();

        let clique = bron_kerbosch(&r, &mut new_p, &mut new_x, edges);

        if clique.len() > maximum_clique.len() {
            maximum_clique = clique;
        }

        // Move the current vertex from p to x for the next iteration.
        p.remove(vertex);
        x.insert(vertex);
    }

    maximum_clique
}

fn find_connected<'a>(
    name: &'a str,
    depth: i32,
    connections: &HashMap<&str, HashSet<&'a str>>,
) -> Vec<String> {
    let mut queue = VecDeque::new();
    queue.push_back((name, depth, vec![name]));
    let mut paths = Vec::new();
    let mut visited: HashSet<&str> = HashSet::new();

    while let Some((node, depth, mut path)) = queue.pop_front() {
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
                queue.push_back((next, depth - 1, path.clone()));
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
        assert_eq!("co,de,ka,ta", process(input)?);
        Ok(())
    }
}
