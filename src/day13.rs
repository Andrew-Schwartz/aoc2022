use std::cmp::Ordering;
use std::fmt::Debug;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, newline};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, tuple};

use crate::utils::number;

type Input = Vec<[Element; 2]>;

#[aoc_generator(day13)]
fn gen(input: &[u8]) -> Input {
    separated_list1(
        tag("\n\n"),
        tuple((
            Element::parse,
            newline,
            Element::parse,
        )).map(|(a, _newline, b)| [a, b]),
    ).parse(input)
        .unwrap()
        .1
}

#[derive(Clone, Debug)]
enum Element {
    Number(u32),
    List(Vec<Element>),
}

impl Eq for Element {}

impl PartialEq<Self> for Element {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Number(s), Self::Number(o)) => s.cmp(o),
            (Self::List(s), Self::List(o)) => {
                let mut s = s.into_iter();
                let mut o = o.into_iter();
                loop {
                    // manually zip so I can see which is shorter lol
                    match (s.next(), o.next()) {
                        (None, Some(_)) => break Ordering::Less,
                        (Some(_), None) => break Ordering::Greater,
                        (None, None) => break Ordering::Equal,
                        (Some(s), Some(o)) => match s.partial_cmp(o).unwrap() {
                            Ordering::Equal => {}
                            ord => break ord,
                        }
                    }
                }
            }
            (s, o) => {
                let s = match s {
                    &Self::Number(s) => Self::List(vec![Self::Number(s)]),
                    list => list.clone(),
                };
                let o = match o {
                    &Self::Number(o) => Self::List(vec![Self::Number(o)]),
                    list => list.clone(),
                };
                s.cmp(&o)
            }
        }
    }
}

impl Element {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        delimited(
            char('['),
            separated_list0(
                char(','),
                alt((
                    Element::parse,
                    number.map(Element::Number),
                )),
            ).map(Element::List),
            char(']'),
        ).parse(input)
    }
}

#[aoc(day13, part1)]
fn part1(input: &Input) -> usize {
    input.into_iter()
        .enumerate()
        .map(|(idx, pair)| (idx + 1, pair))
        .filter(|(_, pair)| pair[0] < pair[1])
        .map(|(idx, _)| idx)
        .sum()
}

#[aoc(day13, part2)]
fn part2(input: &Input) -> u32 {
    let packets = input.into_iter()
        .cloned()
        .flat_map(|e| e.into_iter())
        .collect_vec();
    let [el2, el6] = [2, 6].map(|n| Element::List(vec![Element::List(vec![Element::Number(n)])]));

    let mut before_el2 = 1;
    let mut before_el6 = 2;

    for packet in packets {
        if packet < el6 {
            before_el6 += 1;
            if packet < el2 {
                before_el2 += 1;
            }
        }
    }

    before_el2 * before_el6
}
