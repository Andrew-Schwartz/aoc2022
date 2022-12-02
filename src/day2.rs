use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use crate::utils::CollectArray;

type Input = Vec<[char; 2]>;

#[aoc_generator(day2)]
fn gen(input: &str) -> Input {
    input.lines()
        .map(|l| l.chars()
            .filter(|c| !c.is_ascii_whitespace())
            .take(2)
            .collect_array()
            .unwrap())
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &Input) -> u32 {
    input.iter()
        .map(|&[them, me]| {
            let shape_score = me as u32 - 'W' as u32;
            let win_score = match (me, them) {
                ('X', 'A') => 3,
                ('X', 'B') => 0,
                ('X', 'C') => 6,
                ('Y', 'A') => 6,
                ('Y', 'B') => 3,
                ('Y', 'C') => 0,
                ('Z', 'A') => 0,
                ('Z', 'B') => 6,
                ('Z', 'C') => 3,
                _ => unreachable!(),
            };
            shape_score + win_score
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &Input) -> u32 {
    input.iter()
        .map(|&[them, result]| {
            let win_score = (result as u32 - 'X' as u32) * 3;
            let them_u32 = them as u32 - 'A' as u32;
            let shape_score = match result {
                // lose
                'X' => (if them_u32 == 0 { 3 } else { them_u32 }) - 1,
                'Y' => them_u32,
                'Z' => (them_u32 + 1) % 3,
                _ => unreachable!()
            } + 1;
            win_score + shape_score
        })
        .sum()
}