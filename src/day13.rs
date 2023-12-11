use std::cmp::Ordering;
use std::hint::unreachable_unchecked;

use aoc_runner_derive::aoc;

use crate::utils::{SliceSplitting, TupleIter};

fn less(a: &[u8], b: &[u8]) -> bool {
    let mut a_idx = 0;
    let mut b_idx = 0;
    let mut a_nesting = 0;
    let mut b_nesting = 0;

    loop {
        if a_idx >= a.len() { break true; }
        if b_idx >= b.len() { break false; }
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
                            break false;
                        } else if b_nesting != 0 {
                            break true;
                        }
                    }
                    ord => break ord == Ordering::Less
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
                    break true;
                } else {
                    a_nesting -= 1;
                }
            }
            (_, b']') => {
                if b_nesting == 0 {
                    break false;
                } else {
                    b_nesting -= 1;
                }
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

#[aoc(day13, part1)]
fn part1_unparsed(input: &[u8]) -> usize {
    input.splits(b"\n\n")
        .map(|pair| pair.split_once(b"\n").unwrap())
        .enumerate()
        .filter(|(_, (a, b))| less(a, b))
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
            if less(packet, el6) {
                before_el6 += 1;
                if less(packet, el2) {
                    before_el2 += 1;
                }
            }
        });

    before_el2 * before_el6
}