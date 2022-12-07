use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    IResult,
    multi::separated_list0,
    Parser,
    sequence::tuple,
};

use crate::utils::{ByteLines, number, SliceSplitting};

type Input = (Vec<Vec<u8>>, Vec<Step>);

#[derive(Debug)]
struct Step {
    n: u8,
    from: u8,
    to: u8,
}

impl Step {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        tuple((
            tag("move "),
            number,
            tag(" from "),
            number::<_, u8>,
            tag(" to "),
            number::<_, u8>,
        )).map(|(_, n, _, from, _, to)| Self {
            n,
            from: from - 1,
            to: to - 1,
        }).parse(input)
    }
}

#[aoc_generator(day5)]
fn gen(input: &[u8]) -> Input {
    let (boxes, steps) = input.split_once(b"\n\n").unwrap();

    let (boxes, labels) = boxes.rsplit_once(b"\n").unwrap();

    let len = labels.last().unwrap() - b'0';
    let mut vec = vec![Vec::new(); len as _];
    for line in boxes.lines() {
        let mut chunks = line.iter().enumerate().array_chunks();
        while let Some([_, (i, &c), _, _]) = chunks.next() {
            if c != b' ' { vec[i / 4].push(c) }
        }
        if let &[_, (i, &c), _] = chunks.into_remainder().unwrap().as_slice() {
            vec[i / 4].push(c)
        }
    }
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
            let from = boxes[from as usize].pop().unwrap();
            boxes[to as usize].push(from);
        }
    }
    boxes.into_iter()
        .map(|mut vec| vec.pop().unwrap())
        .map(char::from)
        .collect()
}

#[aoc(day5, part2)]
fn part2((boxes, steps): &Input) -> String {
    let mut boxes = boxes.clone();
    for &Step { n, from, to } in steps {
        let range_start = boxes[from as usize].len() - n as usize;
        for i in range_start..boxes[from as usize].len() {
            let r#box = boxes[from as usize][i];
            boxes[to as usize].push(r#box);
        }
        boxes[from as usize]
            .drain(range_start..);
    }
    boxes.into_iter()
        .map(|mut vec| vec.pop().unwrap())
        .map(char::from)
        .collect()
}