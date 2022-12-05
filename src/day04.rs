use std::ops::RangeInclusive;
use aoc_runner_derive::{aoc, aoc_generator};
use crate::utils::{CollectArray, TupleIter};

type Input = Vec<[RangeInclusive<u32>; 2]>;

#[aoc_generator(day4)]
fn gen(input: &str) -> Input {
    input.lines()
        .map(|l| l.split_once(',')
            .unwrap()
            .tuple_iter()
            .map(|range| {
                let [start, end] = range.split_once('-')
                    .unwrap()
                    .tuple_iter()
                    .map(|n| n.parse().unwrap())
                    .collect_array()
                    .unwrap();
                start..=end
            })
            .collect_array()
            .unwrap())
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &Input) -> usize {
    fn contains(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
        a.start() <= b.start() && a.end() >= b.end()
    }

    input.iter()
        .filter(|[a, b]| contains(a, b) || contains(b, a))
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &Input) -> usize {
    fn overlaps(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
        let (start, end) = (a.start(), a.end());
        b.start() <= start && start <= b.end() || b.start() <= end && end <= b.end()
    }

    input.iter()
        .filter(|[a, b]| overlaps(a, b) || overlaps(b, a))
        .count()
}