use std::hint::unreachable_unchecked;
use aoc_runner_derive::aoc;
use crate::utils::ByteStringExt;

fn gen(input: &[u8]) -> impl Iterator<Item=(u8, u8)> + '_ {
    input.lines()
        .map(|line| match line {
            &[a, b' ', b] => (a, b),
            _ => unsafe { unreachable_unchecked() },
        })
}

#[aoc(day2, part1)]
fn part1(input: &[u8]) -> u32 {
    gen(input).map(|(them, me)| {
        let shape_score = me - b'W';
        #[allow(clippy::match_same_arms)]
        let win_score = match (me, them) {
            (b'X', b'A') => 3,
            (b'X', b'B') => 0,
            (b'X', b'C') => 6,
            (b'Y', b'A') => 6,
            (b'Y', b'B') => 3,
            (b'Y', b'C') => 0,
            (b'Z', b'A') => 0,
            (b'Z', b'B') => 6,
            (b'Z', b'C') => 3,
            _ => unsafe { unreachable_unchecked() },
        };
        shape_score + win_score
    }).map(u32::from).sum()
}

#[aoc(day2, part2)]
fn part2(input: &[u8]) -> u32 {
    gen(input).map(|(them, result)| {
        let win_score = (result - b'X') * 3;
        let them_u32 = them - b'A';
        let shape_score = match result {
            // lose
            b'X' => (if them_u32 == 0 { 3 } else { them_u32 }) - 1,
            b'Y' => them_u32,
            b'Z' => (them_u32 + 1) % 3,
            _ => unsafe { unreachable_unchecked() },
        } + 1;
        win_score + shape_score
    }).map(u32::from).sum()
}