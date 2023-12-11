use std::cmp::{max, min};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::Parser;
use nom::sequence::tuple;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::utils::number;

type Pt = (i32, i32);
type Parsed = Vec<(Pt, Pt)>;

#[aoc_generator(day15)]
fn gen(input: &[u8]) -> Parsed {
    separated_list1(
        newline,
        tuple((
            tag("Sensor at x="),
            number,
            tag(", y="),
            number,
            tag(": closest beacon is at x="),
            number,
            tag(", y="),
            number
        )).map(|(_, x, _, y, _, bx, _, by)| ((x, y), (bx, by))),
    ).parse(input)
        .unwrap()
        .1
}

fn dist((x1, y1): Pt, (x2, y2): Pt) -> u32 {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

fn overlaps([a0, a1]: [i32; 2], [b0, b1]: [i32; 2]) -> bool {
    a1 + 1 >= b0 && b0 >= a0 || b1 + 1 >= a0 && a0 >= b0
}

fn ranges_for_row(input: &Parsed, row: i32) -> Vec<[i32; 2]> {
    // println!("row = {:?}", row);

    let mut ranges = vec![];

    for &(sensor, beacon) in input {
        let d = dist(sensor, beacon) as i32;
        let dy = sensor.1.abs_diff(row) as i32;
        if dy <= d {
            ranges.push([sensor.0 - (d - dy), sensor.0 + (d - dy)])
        }
    }

    loop {
        let mut any_merged = false;

        let mut i = 0;
        while i < ranges.len() {
            let mut j = 0;
            while j < ranges.len() {
                if i == j || i >= ranges.len() || j >= ranges.len() {
                    j += 1;
                    continue;
                }
                // println!("ranges = {:?}", ranges);
                let ri = ranges[i];
                let rj = ranges[j];
                // println!("{ri:?} vs {rj:?}");
                if overlaps(ri, rj) {
                    let [[i0, i1], [j0, j1]] = [max(i, j), min(i, j)].map(|idx| ranges.remove(idx));
                    let new_arr = [min(i0, j0), max(i1, j1)];
                    // println!("new_arr = {:?}", new_arr);
                    ranges.push(new_arr);
                    any_merged = true;
                } else {
                    j += 1;
                }
            }
            i += 1;
        }

        if !any_merged {
            break;
        }
    }

    ranges
}

#[aoc(day15, part1)]
fn part1(input: &Parsed) -> i32 {
    // const Y: i32 = 10;
    const Y: i32 = 2_000_000;

    ranges_for_row(input, Y)
        .into_iter()
        .map(|[l, h]| h - l)
        .sum::<i32>()
}

#[aoc(day15, part2)]
fn part2(input: &Parsed) -> u64 {
    const MIN: i32 = 0;
    // const MAX: i32 = 20;
    const MAX: i32 = 4000000;

    let (y, x) = (MIN..=MAX)
        .into_par_iter()
        .map(|r| (r, ranges_for_row(input, r)))
        .find_map_any(|(r, ranges)| match &ranges[..] {
            &[[l, h]] => {
                (l == MIN + 1).then_some((r, l))
                    .or((h == MAX - 1).then_some((r, h)))
            }
            &[[l1, h1], [l2, h2]] => {
                Some((r, min(h1, h2) + 1))
            }
            all => {
                println!("all = {:?}", all);
                unreachable!()
            }
        })
        .unwrap();

    x as u64 * 4000000 + y as u64
}
