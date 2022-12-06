use std::ops::RangeInclusive;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::bytes::complete::tag;
use nom::{IResult, Parser};
use nom::character::complete::newline;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;

use crate::utils::number;

type Input = Vec<[RangeInclusive<u8>; 2]>;

fn parse_line(input: &str) -> IResult<&str, [RangeInclusive<u8>; 2]> {
    separated_pair(
        separated_pair(number::<u8>, tag("-"), number::<u8>),
        tag(","),
        separated_pair(number::<u8>, tag("-"), number::<u8>),
    ).map(|((a1, a2), (b1, b2))| [a1..=a2, b1..=b2])
        .parse(input)
}

#[aoc_generator(day4)]
fn gen(input: &str) -> Input {
    separated_list0(newline, parse_line).parse(input)
        .unwrap()
        .1
}

#[aoc(day4, part1)]
fn part1(input: &Input) -> usize {
    fn contains(a: &RangeInclusive<u8>, b: &RangeInclusive<u8>) -> bool {
        a.start() <= b.start() && a.end() >= b.end()
    }

    input.iter()
        .filter(|[a, b]| contains(a, b) || contains(b, a))
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &Input) -> usize {
    fn overlaps(a: &RangeInclusive<u8>, b: &RangeInclusive<u8>) -> bool {
        let (start, end) = (a.start(), a.end());
        b.start() <= start && start <= b.end() || b.start() <= end && end <= b.end()
    }

    input.iter()
        .filter(|[a, b]| overlaps(a, b) || overlaps(b, a))
        .count()
}