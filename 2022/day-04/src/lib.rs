use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;
use std::ops::Range;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct RangePair {
    first: Range<u8>,
    second: Range<u8>,
}

impl RangePair {
    pub fn new(first: Range<u8>, second: Range<u8>) -> Self {
        Self { first, second }
    }

    pub fn is_contained(&self) -> bool {
        if self.first.start >= self.second.start && self.first.end <= self.second.end {
            return true;
        }

        if self.second.start >= self.first.start && self.second.end <= self.first.end {
            return true;
        }

        false
    }

    pub fn is_overlapped(&self) -> bool {
        if self.first.start > self.second.end || self.second.start > self.first.end {
            return false;
        }

        if self.second.start > self.first.end || self.first.start > self.second.end {
            return false;
        }

        true
    }
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<RangePair>> {
    separated_list1(line_ending, range_pair).parse(input.trim())
}

fn range_pair(input: &str) -> IResult<&str, RangePair> {
    let (input, (a, b)) = separated_pair(range, tag(","), range).parse(input)?;
    Ok((input, RangePair::new(a, b)))
}

fn range(input: &str) -> IResult<&str, Range<u8>> {
    let (input, (a, b)) = separated_pair(complete::u8, tag("-"), complete::u8).parse(input)?;
    let range = a..b;
    Ok((input, range))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_contained() {
        let pair1 = RangePair::new((2..4), (6..8));
        assert_eq!(false, pair1.is_contained());

        let pair2 = RangePair::new((2..3), (4..5));
        assert_eq!(false, pair2.is_contained());

        let pair3 = RangePair::new((5..7), (7..9));
        assert_eq!(false, pair3.is_contained());

        let pair4 = RangePair::new((2..8), (3..7));
        assert_eq!(true, pair4.is_contained());

        let pair4 = RangePair::new((3..7), (2..8));
        assert_eq!(true, pair4.is_contained());
    }

    #[test]
    fn test_is_overlapped() {
        let pair1 = RangePair::new((2..4), (6..8));
        assert_eq!(false, pair1.is_overlapped());

        let pair2 = RangePair::new((2..3), (4..5));
        assert_eq!(false, pair2.is_overlapped());

        let pair3 = RangePair::new((5..7), (7..9));
        assert_eq!(true, pair3.is_overlapped());

        let pair4 = RangePair::new((2..8), (3..7));
        assert_eq!(true, pair4.is_overlapped());

        let pair4 = RangePair::new((3..7), (2..8));
        assert_eq!(true, pair4.is_overlapped());
    }
}
