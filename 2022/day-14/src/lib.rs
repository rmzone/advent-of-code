use glam::IVec2;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;
use std::collections::HashMap;
use std::fmt::Display;

pub mod part1;
pub mod part2;

const START: IVec2 = IVec2::new(500, 0);

#[derive(Debug)]
pub enum Material {
    Rock,
    Sand,
}

#[derive(Debug)]
pub struct Cave {
    mat: HashMap<IVec2, Material>,
    bottom: i32,
}

pub const DIRECTIONS: [IVec2; 3] = [IVec2::new(0, 1), IVec2::new(-1, 1), IVec2::new(1, 1)];

impl Cave {
    #[must_use]
    pub fn new(paths: &Vec<Path>) -> Cave {
        // build map
        let mut mat = HashMap::new();
        for path in paths.iter() {
            Self::fill_path(&mut mat, path);
        }

        // find bottom
        let bottom = mat.iter().map(|(k, _)| k.y).max().unwrap();

        Cave { mat, bottom }
    }

    pub fn add_bottom(&mut self) {
        self.bottom += 2;
        let x_min = self.mat.iter().map(|(k, _)| k.x).min().unwrap() - 1000;
        let x_max = self.mat.iter().map(|(k, _)| k.x).max().unwrap() + 1000;
        let path = Path {
            nodes: vec![
                IVec2::new(x_min, self.bottom),
                IVec2::new(x_max, self.bottom),
            ],
        };

        Self::fill_path(&mut self.mat, &path);
    }

    fn fill_path(mat: &mut HashMap<IVec2, Material>, path: &Path) {
        for (start, end) in path.nodes.iter().tuple_windows() {
            let delta = end - start;
            let dx = if delta.x == 0 {
                0
            } else {
                delta.x / delta.x.abs()
            };
            let dy = if delta.y == 0 {
                0
            } else {
                delta.y / delta.y.abs()
            };
            let d = IVec2::new(dx, dy);

            let mut current = *start;
            mat.entry(current.clone()).or_insert(Material::Rock);
            while current != *end {
                current += d;
                mat.entry(current.clone()).or_insert(Material::Rock);
            }
        }
    }

    pub fn drop_sand(&mut self) -> bool {
        let mut sand = START.clone();
        let mut is_settled = false;

        while sand.y <= self.bottom && !is_settled {
            let mut not_done = true;
            for dir in &DIRECTIONS {
                let new_sand = sand + dir;
                if !self.mat.contains_key(&new_sand) {
                    sand = new_sand;
                    not_done = false;
                    break;
                }
            }

            is_settled = not_done;
        }

        if sand == START {
            self.mat.entry(sand).or_insert(Material::Sand);
            return false;
        }

        if is_settled {
            self.mat.entry(sand).or_insert(Material::Sand);
            return true;
        }

        false
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_min = self.mat.iter().map(|(k, _)| k.x).min().unwrap() - 2;
        let x_max = self.mat.iter().map(|(k, _)| k.x).max().unwrap() + 2;

        let mut output = String::new();
        output.push('\n');

        for y in 0..=self.bottom {
            for x in x_min..x_max {
                match self.mat.get(&IVec2::new(x, y)) {
                    None => {
                        output.push('.');
                    }
                    Some(material) => match material {
                        Material::Rock => {
                            output.push('#');
                        }
                        Material::Sand => {
                            output.push('o');
                        }
                    },
                }
            }
            output.push('\n');
        }

        write!(f, "{}", output)
    }
}

#[derive(Debug)]
pub struct Path {
    nodes: Vec<IVec2>,
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Path>> {
    separated_list1(line_ending, path).parse(input)
}

fn path(input: &str) -> IResult<&str, Path> {
    separated_list1(tag(" -> "), node)
        .map(|paths| Path { nodes: paths })
        .parse(input)
}

fn node(input: &str) -> IResult<&str, IVec2> {
    map(
        separated_pair(
            nom::character::complete::i32,
            tag(","),
            nom::character::complete::i32,
        ),
        |(a, b)| IVec2::new(a, b),
    )
    .parse(input)
}
