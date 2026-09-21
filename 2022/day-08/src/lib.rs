use common::square_grid::SquareGrid;

pub mod part1;
pub mod part2;

/// Calculate the viewing distance in each direction
pub fn scenic_score(x: usize, y: usize, trees: &SquareGrid<u8>) -> usize {
    let size = trees.size();
    let current_tree = trees[(x, y)];

    let mut score_north = 0;
    for py in (0..y).rev() {
        score_north += 1;
        let north_tree = trees[(x, py)];
        if north_tree >= current_tree {
            break;
        }
    }

    let mut score_south = 0;
    for py in y + 1..size {
        score_south += 1;
        let south_tree = trees[(x, py)];
        if south_tree >= current_tree {
            break;
        }
    }

    let mut score_west = 0;
    for px in (0..x).rev() {
        score_west += 1;
        let west_tree = trees[(px, y)];
        if west_tree >= current_tree {
            break;
        }
    }

    let mut score_east = 0;
    for px in x + 1..size {
        score_east += 1;
        let east_tree = trees[(px, y)];
        if east_tree >= current_tree {
            break;
        }
    }

    score_north * score_south * score_west * score_east
}

/// Check if a tree at the given position is visible
pub fn is_visible(x: usize, y: usize, trees: &SquareGrid<u8>) -> bool {
    let size = trees.size();
    let current_tree = trees[(x, y)];

    let is_visible_north = (0..y).rev().all(|py| trees[(x, py)] < current_tree);

    let is_visible_south = (y + 1..size).all(|py| trees[(x, py)] < current_tree);

    let is_visible_west = (0..x).rev().all(|px| trees[(px, y)] < current_tree);

    let is_visible_east = (x + 1..size).all(|px| trees[(px, y)] < current_tree);

    is_visible_north | is_visible_south | is_visible_west | is_visible_east
}

pub fn parse_input(input: &str) -> SquareGrid<u8> {
    // assumes the input is a square
    let size = input.lines().next().unwrap().len();
    let mut grid = SquareGrid::new(size);

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid[(x, y)] = ch.to_digit(10).unwrap() as u8;
        }
    }

    grid
}
