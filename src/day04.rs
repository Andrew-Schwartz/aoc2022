use std::ops::Range;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{IResult, Parser};
use nom::character::complete::{char, newline};
use nom::multi::separated_list0;
use nom::sequence::separated_pair;

use crate::utils::number;

type Input = Vec<[Range<u8>; 2]>;

fn parse_line(input: &[u8]) -> IResult<&[u8], [Range<u8>; 2]> {
    separated_pair(
        separated_pair(number, char('-'), number),
        char(','),
        separated_pair(number, char('-'), number),
    ).map(|((a1, a2), (b1, b2))| [a1..a2, b1..b2])
        .parse(input)
}

#[aoc_generator(day4)]
fn gen(input: &[u8]) -> Input {
    separated_list0(newline, parse_line).parse(input)
        .unwrap()
        .1
}

#[aoc(day4, part1)]
fn part1(input: &Input) -> usize {
    fn contains(a: &Range<u8>, b: &Range<u8>) -> bool {
        a.start <= b.start && a.end >= b.end
    }

    input.iter()
        .filter(|[a, b]| contains(a, b) || contains(b, a))
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &Input) -> usize {
    fn overlaps(a: &Range<u8>, b: &Range<u8>) -> bool {
        let (start, end) = (a.start, a.end);
        b.start <= start && start <= b.end || b.start <= end && end <= b.end
    }

    input.iter()
        .filter(|[a, b]| overlaps(a, b) || overlaps(b, a))
        .count()
}