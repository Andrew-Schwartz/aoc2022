use std::collections::binary_heap::BinaryHeap;

use aoc_runner_derive::{aoc, aoc_generator};

type Data<T> = BinaryHeap<T>;

#[aoc_generator(day1)]
fn gen(input: &str) -> Data<u32> {
    input.split("\n\n")
        .map(|group| group.lines()
            .map(|line| line.parse::<u32>().unwrap())
            .sum())
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &Data<u32>) -> u32 {
    *input.iter().next().unwrap()
}

#[aoc(day1, part2)]
fn part2(input: &Data<u32>) -> u32 {
    input.iter().take(3).sum()
}