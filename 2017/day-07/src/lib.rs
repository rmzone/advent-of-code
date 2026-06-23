use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha1, line_ending, space0};
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::IResult;
use nom::Parser;
use std::collections::{HashMap, HashSet};

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Tower<'a> {
    pub name: &'a str,
    pub weight: i32,
    pub total_weight: i32,
    pub discs: Option<Vec<&'a str>>,
}

pub fn find_bottom<'a>(tower_map: &mut HashMap<&str, Tower<'a>>) -> &'a str {
    let mut possible_bottoms = HashSet::new();
    tower_map.values().for_each(|tower| {
        possible_bottoms.insert(tower.name);
    });

    for tower in tower_map.values() {
        // info!("{:?}", &tower);

        if let Some(discs) = &tower.discs {
            for disk in discs {
                if possible_bottoms.contains(disk) {
                    possible_bottoms.remove(disk);
                }
            }
        }
    }

    // info!("Possible Bottoms: {:?}", possible_bottoms);
    let bottom = possible_bottoms.iter().map(|&name| name).last().unwrap();
    bottom
}

pub fn parse_input(input: &str) -> IResult<&str, HashMap<&str, Tower<'_>>> {
    let (input, towers) = separated_list1(line_ending, parse_tower).parse(input)?;
    let map = towers.into_iter().map(|t| (t.name, t)).collect();
    Ok((input, map))
}

fn parse_tower(input: &str) -> IResult<&str, Tower<'_>> {
    let (input, name) = alpha1(input)?;
    let (input, _) = space0(input)?;
    let (input, weight) = delimited(tag("("), complete::i32, tag(")")).parse(input)?;
    let (input, _) = opt(tag(" -> ")).parse(input)?;
    let (input, disks) = opt(parse_disks).parse(input)?;
    Ok((
        input,
        Tower {
            name,
            weight,
            total_weight: 0,
            discs: disks,
        },
    ))
}

fn parse_disks(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(", "), alpha1).parse(input)
}
