use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::anychar;
use nom::multi::separated_list1;
use nom::IResult;
use nom::Parser;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Move>> {
    separated_list1(tag(","), parse_move).parse(input)
}

pub fn spin(x: usize, programs: &mut Vec<char>) {
    programs.rotate_right(x);
}

pub fn exchange(x: usize, y: usize, programs: &mut Vec<char>) {
    programs.swap(x, y);
}

pub fn partner(a: char, b: char, programs: &mut Vec<char>) {
    let x = programs.iter().position(|&x| x == a).unwrap();
    let y = programs.iter().position(|&y| y == b).unwrap();
    programs.swap(x, y);
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, move_type) = anychar(input)?;

    let (input, m) = match move_type {
        's' => parse_spin(input)?,
        'x' => parse_exchange(input)?,
        'p' => parse_partner(input)?,
        _ => panic!("unknown move type {}", move_type),
    };

    Ok((input, m))
}

fn parse_spin(input: &str) -> IResult<&str, Move> {
    let (input, spin) = complete::usize(input)?; // todo: use map?
    Ok((input, Move::Spin(spin)))
}

fn parse_exchange(input: &str) -> IResult<&str, Move> {
    let (input, a) = complete::usize(input)?; // todo: use map?
    let (input, _) = tag("/")(input)?;
    let (input, b) = complete::usize(input)?; // todo: use map?
    Ok((input, Move::Exchange(a, b)))
}

fn parse_partner(input: &str) -> IResult<&str, Move> {
    let (input, a) = anychar(input)?; // todo: use map?
    let (input, _) = tag("/")(input)?;
    let (input, b) = anychar(input)?; // todo: use map?
    Ok((input, Move::Partner(a, b)))
}
