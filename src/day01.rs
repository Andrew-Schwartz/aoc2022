use aoc_runner_derive::aoc;
use arrayvec::ArrayVec;

use crate::utils::{ByteStringExt, ParseNumber, SliceSplitting, TryRemove};

type Input = [u8];

fn gen(input: &Input) -> impl Iterator<Item=u32> + '_ {
    input.splits(b"\n\n")
        .map(|group| group.lines()
            .map(ParseNumber::parse_number::<u32>)
            .map(Option::unwrap)
            .sum())
}

#[aoc(day1, part1)]
fn part1(input: &Input) -> u32 {
    gen(input)
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
fn part2(input: &Input) -> u32 {
    gen(input)
        .fold(ArrayVec::<_, 3>::new(), |mut arr, cals| {
            let idx = arr.partition_point(|&n| n > cals);
            if idx != 3 {
                arr.try_remove(3 - 1);
                arr.insert(idx, cals);
            }
            arr
        })
        .into_iter()
        .sum()
}