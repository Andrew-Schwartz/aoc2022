use aoc_runner_derive::{aoc, aoc_generator};
use nom::{IResult, Parser};
use nom::character::complete::{char, newline};
use nom::multi::separated_list0;
use nom::sequence::separated_pair;

use crate::utils::{number, TupleIter};

type Input = ([u8; 2], [u8; 2]);

fn parse_line(input: &[u8]) -> IResult<&[u8], Input> {
    separated_pair(
        separated_pair(number, char('-'), number),
        char(','),
        separated_pair(number, char('-'), number),
    ).map(|tups| tups.tuple_map(|(s, e)| [s, e]))
        .parse(input)
}

// // lol how is the Vec faster
#[aoc_generator(day4)]
fn gen(input: &[u8]) -> Vec<Input> {
    separated_list0(newline, parse_line).parse(input)
        .unwrap()
        .1
}

#[aoc(day4, part1)]
fn part1(input: &[Input]) -> usize {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn contains(a: &[u8; 2], b: &[u8; 2]) -> bool {
        a[0] <= b[0] && a[1] >= b[1]
    }

    input.iter()
        .filter(|(a, b)| contains(a, b) || contains(b, a))
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &[Input]) -> usize {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn overlaps(a: &[u8; 2], b: &[u8; 2]) -> bool {
        let (start, end) = (a[0], a[1]);
        b[0] <= start && start <= b[1] || b[0] <= end && end <= b[1]
    }

    input.iter()
        .filter(|(a, b)| overlaps(a, b) || overlaps(b, a))
        .count()
}