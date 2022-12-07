use std::ops::Range;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{IResult, Parser};
use nom::character::complete::{char, newline};
use nom::multi::separated_list0;
use nom::sequence::separated_pair;

use crate::utils::{number, TupleIter};

type Input = (Range<u8>, Range<u8>);

fn parse_line(input: &[u8]) -> IResult<&[u8], Input> {
    separated_pair(
        separated_pair(number, char('-'), number),
        char(','),
        separated_pair(number, char('-'), number),
    ).map(|tups| tups.tuple_map(|(s, e)| s..e))
        .parse(input)
}

// lol how is the Vec faster
#[aoc_generator(day4)]
fn gen(input: &[u8]) -> Vec<Input> {
    separated_list0(newline, parse_line).parse(input)
        .unwrap()
        .1
}

#[aoc(day4, part1)]
fn part1(input: &Vec<Input>) -> usize {
    fn contains(a: &Range<u8>, b: &Range<u8>) -> bool {
        a.start <= b.start && a.end >= b.end
    }

    input.into_iter()
        .filter(|(a, b)| contains(a, b) || contains(b, a))
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &Vec<Input>) -> usize {
    fn overlaps(a: &Range<u8>, b: &Range<u8>) -> bool {
        let (start, end) = (a.start, a.end);
        b.start <= start && start <= b.end || b.start <= end && end <= b.end
    }

    input.into_iter()
        .filter(|(a, b)| overlaps(a, b) || overlaps(b, a))
        .count()
}