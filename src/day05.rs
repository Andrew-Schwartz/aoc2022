use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    IResult,
    multi::separated_list0,
    Parser,
    sequence::tuple
};
use utils::number;
use crate::utils;

type Input = (Vec<Vec<char>>, Vec<Step>);

#[derive(Debug)]
struct Step {
    n: usize,
    from: usize,
    to: usize,
}

impl Step {
    fn parse(input: &str) -> IResult<&str, Self> {
        tuple((
            tag("move "),
            number::<usize>,
            tag(" from "),
            number::<usize>,
            tag(" to "),
            number::<usize>,
        )).map(|(_, n, _, from, _, to)| Self {
            n,
            from: from - 1,
            to: to - 1,
        }).parse(input)
    }
}

#[aoc_generator(day5)]
fn gen(input: &str) -> Input {
    let (boxes, steps) = input.split_once("\n\n").unwrap();

    let (boxes, labels) = boxes.rsplit_once('\n').unwrap();
    let len = labels.chars().last()
        .unwrap()
        .to_digit(10)
        .unwrap()
        as usize;
    let mut vec = vec![Vec::new(); len];
    boxes.lines()
        .for_each(|line| line.chars()
            .chunks(4)
            .into_iter()
            .enumerate()
            .map(|(i, mut c)| (i, c.nth(1).unwrap()))
            .filter(|&(_, c)| c != ' ')
            .for_each(|(i, c)| vec[i].push(c)));
    vec.iter_mut()
        .for_each(|vec| vec.reverse());

    let steps = separated_list0(line_ending, Step::parse).parse(steps).unwrap().1;

    (vec, steps)
}

#[aoc(day5, part1)]
fn part1((boxes, steps): &Input) -> String {
    let mut boxes = boxes.clone();
    for &Step { n, from, to } in steps {
        for _ in 0..n {
            let from = boxes[from].pop().unwrap();
            boxes[to].push(from);
        }
    }
    boxes.into_iter()
        .map(|mut vec| vec.pop().unwrap())
        .collect()
}

#[aoc(day5, part2)]
fn part2((boxes, steps): &Input) -> String {
    let mut boxes = boxes.clone();
    for &Step { n, from, to } in steps {
        let range_start = boxes[from].len() - n;
        let crates = boxes[from]
            .drain(range_start..)
            .collect_vec();
        boxes[to].extend(crates);
    }
    boxes.into_iter()
        .map(|mut vec| vec.pop().unwrap())
        .collect()
}