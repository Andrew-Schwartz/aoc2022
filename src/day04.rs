use std::array;
use std::simd::{Mask, Simd, SimdPartialOrd, ToBitMask};

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{IResult, Parser};
use nom::character::complete::{char, newline};
use nom::multi::separated_list0;
use nom::sequence::separated_pair;

use crate::utils::{number, TupleIter};

// type Input = (Range<u8>, Range<u8>);

fn parse_line(input: &[u8]) -> IResult<&[u8], ([u8; 2], [u8; 2])> {
    separated_pair(
        separated_pair(number, char('-'), number),
        char(','),
        separated_pair(number, char('-'), number),
    ).map(|tups| tups.tuple_map(|(s, e)| [s, e]))
        .parse(input)
}

// lol how is the Vec faster
#[aoc_generator(day4)]
fn gen(input: &[u8]) -> Vec<([u8; 2], [u8; 2])> {
    separated_list0(newline, parse_line).parse(input)
        .unwrap()
        .1
    // .into_iter()
    // .unzip()
}

#[aoc(day4, part1)]
fn part1(input: &Vec<([u8; 2], [u8; 2])>) -> u32 {
    const N: usize = 64;

    fn contains(a: &[u8; 2], b: &[u8; 2]) -> bool {
        a[0] <= b[0] && a[1] >= b[1]
    }

    fn scontains(a0: Simd<u8, N>, a1: Simd<u8, N>, b0: Simd<u8, N>, b1: Simd<u8, N>) -> Mask<i8, N> {
        a0.simd_le(b0) & a1.simd_ge(b1)
    }

    let mut chunks = input.array_chunks::<N>();
    let count: u32 = chunks.by_ref()
        .map(|chunk| (
            array::from_fn(|i| chunk[i].0[0]),
            array::from_fn(|i| chunk[i].0[1]),
            array::from_fn(|i| chunk[i].1[0]),
            array::from_fn(|i| chunk[i].1[1]),
        ))
        .map(|tuple| tuple.tuple_map(Simd::from_array))
        .map(|(a0, a1, b0, b1)| scontains(a0, a1, b0, b1) | scontains(b0, b1, a0, a1))
        .map(Mask::to_bitmask)
        .map(u64::count_ones)
        .sum();

    let rem = chunks.remainder().iter()
        .filter(|(a, b)| contains(a, b) || contains(b, a))
        .count();

    count + (rem as u32)
}

// #[aoc(day4, part2)]
// fn part2(input: &[Input]) -> usize {
//     fn overlaps(a: &Range<u8>, b: &Range<u8>) -> bool {
//         let (start, end) = (a.start, a.end);
//         b.start <= start && start <= b.end || b.start <= end && end <= b.end
//     }
// 
//     fn simd_overlaps(a: u8x64, b: u8x64) -> bool {
// 
//     }
// 
//     let chunks = input
//         .array_chunks::<4>();
//     chunks.by_ref()
//         .map(|inputs| {
//             let x = inputs
//                 .map(|(a, b)|);
//             x
//         }
//         )
// 
//     // input.iter()
//     //     .filter(|(a, b)| overlaps(a, b) || overlaps(b, a))
//     //     .count()
// }