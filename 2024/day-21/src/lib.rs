use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::OnceLock;

pub mod part1;
pub mod part2;

// The next two functions basically find all the possible shortest paths between
// any two points on each of the keypads. Because the combinations are small, we
// can pre-compute.
//
// We use OnceLock here and in the next function (previously would be
// lazy_static) so we can use the cached macro.  There may be an easier way to
// do this but it is nice that the cached macro has a reduced parameter list.

pub fn numeric_paths() -> &'static HashMap<(char, char), Vec<String>> {
    static NUMERIC_PATHS: OnceLock<HashMap<(char, char), Vec<String>>> = OnceLock::new();
    NUMERIC_PATHS.get_or_init(|| {
        let numeric_keypad = vec![
            ('9', vec![('6', 'v'), ('8', '<')]),
            ('8', vec![('7', '<'), ('9', '>'), ('5', 'v')]),
            ('7', vec![('4', 'v'), ('8', '>')]),
            ('6', vec![('9', '^'), ('5', '<'), ('3', 'v')]),
            ('5', vec![('8', '^'), ('4', '<'), ('2', 'v'), ('6', '>')]),
            ('4', vec![('7', '^'), ('1', 'v'), ('5', '>')]),
            ('3', vec![('6', '^'), ('2', '<'), ('A', 'v')]),
            ('2', vec![('5', '^'), ('0', 'v'), ('1', '<'), ('3', '>')]),
            ('1', vec![('4', '^'), ('2', '>')]),
            ('0', vec![('2', '^'), ('A', '>')]),
            ('A', vec![('3', '^'), ('0', '<')]),
        ]
        .into_iter()
        .collect::<HashMap<char, Vec<(char, char)>>>();

        // For each combination of buttons, find the shortest paths between them.
        numeric_keypad
            .keys()
            .cartesian_product(numeric_keypad.keys())
            .map(|(&a, &b)| ((a, b), find_shortest_paths(&numeric_keypad, a, b)))
            .collect()
    })
}

pub fn direction_paths() -> &'static HashMap<(char, char), Vec<String>> {
    static DIRECTION_PATHS: OnceLock<HashMap<(char, char), Vec<String>>> = OnceLock::new();
    DIRECTION_PATHS.get_or_init(|| {
        let direction_keypad = vec![
            ('^', vec![('v', 'v'), ('A', '>')]),
            ('v', vec![('<', '<'), ('>', '>'), ('^', '^')]),
            ('<', vec![('v', '>')]),
            ('>', vec![('v', '<'), ('A', '^')]),
            ('A', vec![('^', '<'), ('>', 'v')]),
        ]
        .into_iter()
        .collect::<HashMap<char, Vec<(char, char)>>>();

        // For each combination of buttons, find the shortest paths between them.
        direction_keypad
            .keys()
            .cartesian_product(direction_keypad.keys())
            .map(|(&a, &b)| ((a, b), find_shortest_paths(&direction_keypad, a, b)))
            .collect()
    })
}

fn find_shortest_paths(
    neighbors: &HashMap<char, Vec<(char, char)>>,
    start: char,
    end: char,
) -> Vec<String> {
    // We use a queue here to do a breadth-first search of the keypad.
    let mut queue = VecDeque::new();
    queue.push_back((start, Vec::new(), HashSet::new()));

    // Track the paths we've found so far and the length of the shortest path.
    let mut paths = Vec::new();
    let mut lowest = usize::MAX;

    // While we have nodes to visit, keep looking for the end.
    while let Some((node, path, mut visited)) = queue.pop_front() {
        // If we found the end, add the path to the list of paths if it's part of the lowest.
        if node == end {
            if path.len() <= lowest {
                lowest = path.len();
                paths.push(path.iter().collect::<String>());
            }
            continue; // There may be more.
        }

        // Check to see if we have already visited this node. If we are
        // continuing, add it to our visited set.
        if visited.contains(&node) {
            continue;
        }

        visited.insert(node);

        // For each neighbor create a new path that includes it and add it to out queue.
        // todo: lots of cloning. It this absolutely required?
        for (next, dir) in neighbors.get(&node).unwrap() {
            let mut path = path.clone();
            path.push(*dir);
            queue.push_back((*next, path.clone(), visited.clone()));
        }
    }

    paths
}

#[cached]
pub fn find_shortest_sequence(sequence: String, depth: usize, is_numeric: bool) -> usize {
    // Pick the right keypad paths.
    let paths = if is_numeric {
        numeric_paths()
    } else {
        direction_paths()
    };

    // We want to find the path from each button to the next. All robots start
    // at 'A', so we prefix the windows with that.
    ("A".to_string() + &sequence)
        .chars()
        .tuple_windows()
        .map(|(a, b)| {
            let shortest_paths = paths.get(&(a, b)).unwrap();
            match depth {
                // If we've reached the end, we just use the shortest path length.
                0 => shortest_paths[0].len() + 1,
                // Otherwise, we need to find the smallest path among all the paths.
                _ => shortest_paths
                    .iter()
                    .cloned()
                    .map(|mut path| {
                        // We put and 'A' at the end because they'll need to
                        // hit the 'A' button to tell the next in line to push their button.
                        path.push('A');
                        find_shortest_sequence(path, depth - 1, false)
                    })
                    .min()
                    .unwrap(),
            }
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pre_compile_numeric_paths() {
        let paths = numeric_paths();

        for (key, path) in paths.iter() {
            println!("('{}', '{}'): {:?}", key.0, key.1, path);
        }

        assert_eq!(paths.len(), 121);
    }

    #[test]
    fn test_pre_compile_direction_paths() {
        let paths = direction_paths();

        for (key, path) in paths.iter() {
            println!("('{}', '{}'): {:?}", key.0, key.1, path);
        }

        assert_eq!(paths.len(), 25);
    }
}
