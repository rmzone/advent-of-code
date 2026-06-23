use common::custom_error::AocError;
use indicatif::{ParallelProgressIterator, ProgressIterator};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete;
use nom::character::complete::{digit1, line_ending, newline, space0, space1};
use nom::combinator::{peek, rest};
use nom::multi::{count, fold_many1, many1, separated_list1};
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};
use nom::{IResult, Parser};
use nom_supreme::ParserExt;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashSet;
use std::ops::Range;

#[derive(Debug)]
struct Almanac {
    seeds: Vec<Range<u64>>,
    maps: Vec<SeedMap>,
}

#[derive(Debug)]
struct SeedMap {
    mappings: Vec<(Range<u64>, Range<u64>)>,
}

impl SeedMap {
    fn translate(&self, source: u64) -> u64 {
        let valid_mapping = self
            .mappings
            .iter()
            .find(|(source_range, _)| source_range.contains(&source));

        let Some((source_range, destination_range)) = valid_mapping else {
            return source;
        };

        let offset = source - source_range.start;

        destination_range.start + offset
    }
}

fn parse_set(input: &str) -> IResult<&str, Vec<u64>> {
    fold_many1(
        terminated(complete::u64, space0),
        Vec::new,
        |mut acc: Vec<_>, item| {
            acc.push(item);
            acc
        },
    )(input)
}

fn parse_seed_range(input: &str) -> IResult<&str, Range<u64>> {
    let (input, (start, length)) =
        tuple((complete::u64, complete::u64.preceded_by(tag(" "))))(input)?;

    Ok((input, (start..(start + length))))
}

fn parse_seed_list(input: &str) -> IResult<&str, Vec<Range<u64>>> {
    tag("seeds: ")
        .precedes(separated_list1(
            space1,
            separated_pair(complete::u64, tag(" "), complete::u64)
                .map(|(start, offset)| start..(start + offset)),
        ))
        .parse(input)
}

fn line(input: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
    let (input, (destination, source, num)) = tuple((
        complete::u64,
        complete::u64.preceded_by(tag(" ")),
        complete::u64.preceded_by(tag(" ")),
    ))(input)?;

    Ok((
        input,
        (source..(source + num), destination..(destination + num)),
    ))
}

fn parse_seed_map(input: &str) -> IResult<&str, SeedMap> {
    take_until("map:")
        .precedes(tag("map:"))
        .precedes(many1(line_ending.precedes(line)).map(|mappings| SeedMap { mappings }))
        .parse(input)
}

fn parse_seed_maps(input: &str) -> IResult<&str, Vec<SeedMap>> {
    many1(parse_seed_map)(input)
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let (input, seeds) = parse_seed_list(input).unwrap();
    let (input, maps) = parse_seed_maps(input).unwrap();

    // build a list of the locations for each seed
    // take the lowest

    Ok((
        input,
        Almanac {
            seeds: seeds,
            maps: maps,
        },
    ))
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, almanac) = parse_almanac(input).expect("should parse");

    // dbg!(&almanac);

    // too slow
    // let mut results: Vec<u64> = vec![];
    //
    // for range in almanac.seeds {
    //     for seed in range {
    //         let mut location = seed;
    //         for map in almanac.maps.iter() {
    //             location = map.translate(location);
    //         }
    //         results.push(location);
    //     }
    // }
    //
    // let result = results.iter().min().unwrap();
    //
    // Ok(result.to_string())

    let locations = almanac
        .seeds
        .iter()
        .flat_map(|range| range.clone().into_iter())
        .collect::<Vec<u64>>();

    let locations = locations
        .into_par_iter()
        .progress()
        .map(|seed| {
            almanac
                .maps
                .iter()
                .fold(seed, |seed, map| map.translate(seed))
        })
        .collect::<Vec<u64>>();

    Ok(locations
        .iter()
        .min()
        .expect("should have a minimum location value")
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!("46", process(input)?);
        Ok(())
    }
}
