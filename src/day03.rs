use std::collections::BTreeSet;

use aoc_runner_derive::aoc;

use crate::utils::{CollectArray, TupleIter};

fn score(c: char) -> u32 {
    match c {
        lower @ 'a'..='z' => lower as u32 - 'a' as u32 + 1,
        upper @ 'A'..='Z' => upper as u32 - 'A' as u32 + 27,
        _ => unreachable!(),
    }
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    input.lines()
        .map(|l| l.split_at(l.len() / 2)
            .tuple_iter()
            .map(|str| str.chars().collect::<BTreeSet<_>>())
            .collect_array()
            .unwrap())
        .map(|[a, b]| a.into_iter()
            .find(|item| b.contains(item))
            .unwrap())
        .map(score)
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    input.lines()
        .array_chunks::<3>()
        .map(|lines| lines.into_iter()
            .map(|line| line.chars().collect::<BTreeSet<_>>())
            .collect_array()
            .unwrap())
        .map(|[a, b, c]| a.into_iter()
            .find(|item| b.contains(item) && c.contains(item))
            .unwrap())
        .map(score)
        .sum()
}
