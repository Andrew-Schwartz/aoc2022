use std::ops::BitOr;

use aoc_runner_derive::aoc;
use crate::utils::TupleIter;

fn score(c: &u8) -> u64 {
    match c {
        lower @ b'a'..=b'z' => lower - b'a' + 1,
        upper @ b'A'..=b'Z' => upper - b'A' + 27,
        _ => unreachable!(),
    }.into()
}

fn to_bits(chars: &[u8]) -> u64 {
    chars.iter()
        .map(score)
        .map(|shift| 1 << shift)
        .reduce(BitOr::bitor)
        .unwrap()
}

#[aoc(day3, part1)]
fn part1(input: &[u8]) -> u32 {
    input.split(|&c| c == b'\n')
        .map(|l| l.split_at(l.len() / 2).tuple_map(to_bits))
        .map(|(a, b)| a & b)
        .map(u64::trailing_zeros)
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &[u8]) -> u32 {
    input.split(|&c| c == b'\n')
        .array_chunks::<3>()
        .map(|lines| lines.map(to_bits))
        .map(|[a, b, c]| a & b & c)
        .map(u64::trailing_zeros)
        .sum()
}
