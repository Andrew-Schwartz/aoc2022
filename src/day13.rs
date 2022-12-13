use std::cmp::Ordering;
use std::fmt::Debug;
use std::hint::unreachable_unchecked;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, newline};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, tuple};

use crate::utils::{number, SliceSplitting, TupleIter};

fn cmp(a: &[u8], b: &[u8]) -> Ordering {
    let mut a_idx = 0;
    let mut b_idx = 0;
    let mut a_nesting = 0;
    let mut b_nesting = 0;

    let ordering = loop {
        if a_idx >= a.len() { break Ordering::Less; }
        if b_idx >= b.len() { break Ordering::Greater; }
        match (a[a_idx], b[b_idx]) {
            (a_dig @ b'0'..=b'9', b_dig @ b'0'..=b'9') => {
                a_idx += 1;
                let mut a_n = a_dig - b'0';
                // at most 2 digits
                if a[a_idx].is_ascii_digit() {
                    a_n = a_n * 10 + a[a_idx] - b'0';
                    a_idx += 1;
                }
                b_idx += 1;
                let mut b_n = b_dig - b'0';
                // at most 2 digits
                if b[b_idx].is_ascii_digit() {
                    b_n = b_n * 10 + b[b_idx] - b'0';
                    b_idx += 1;
                }
                match a_n.cmp(&b_n) {
                    Ordering::Equal => {
                        if a_nesting != 0 {
                            break Ordering::Greater
                        } else if b_nesting != 0 {
                            break Ordering::Less
                        }
                    }
                    ord => break ord
                }
            }
            (b'[', b'0'..=b'9') => {
                a_idx += 1;
                a_nesting += 1;
            }
            (b'0'..=b'9', b'[') => {
                b_idx += 1;
                b_nesting += 1;
            }
            (b'[', b'[')
            | (b']', b']')
            | (b',', b',') => {
                a_idx += 1;
                b_idx += 1;
            }
            (b']', _) => {
                if a_nesting == 0 {
                    break Ordering::Less;
                } else {
                    a_nesting -= 1;
                }
            }
            (_, b']') => {
                if b_nesting == 0 {
                    break Ordering::Greater;
                } else {
                    b_nesting -= 1;
                }
            }
            _ => unsafe { unreachable_unchecked() },
        }
    };
    ordering
}

#[aoc(day13, part1)]
fn part1_unparsed(input: &[u8]) -> usize {
    input.splits(b"\n\n")
        .map(|pair| pair.split_once(b"\n").unwrap())
        .enumerate()
        .filter(|(_, (a, b))| cmp(a, b) == Ordering::Less)
        .map(|(idx, _)| idx + 1)
        .sum()
}

#[aoc(day13, part2)]
fn part2_unparsed(input: &[u8]) -> usize {
    let [el2, el6] = [&b"[[2]]"[..], &b"[[6]]"[..]];
    let mut before_el2 = 1;
    let mut before_el6 = 2;

    input.splits(b"\n\n")
        .map(|pair| pair.split_once(b"\n").unwrap())
        .flat_map(|tup| tup.tuple_iter())
        .for_each(|packet| {
            if cmp(packet, el6) == Ordering::Less {
                before_el6 += 1;
                if cmp(packet, el2) == Ordering::Less {
                    before_el2 += 1;
                }
            }
        });

    before_el2 * before_el6
}