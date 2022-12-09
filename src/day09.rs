use std::collections::HashSet;
use std::hash::{BuildHasherDefault, Hasher};
use std::ptr;
use std::simd::{i16x2, SimdInt, SimdOrd, SimdPartialEq};

use aoc_runner_derive::aoc;

use crate::utils::{ByteStringExt, ParseNumber, TupleIter};

type Point = i16x2;
type Input = (Point, u8);

#[derive(Default, Debug)]
struct NullHasher(u64);

impl Hasher for NullHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        debug_assert_eq!(self.0, 0);
        debug_assert_eq!(bytes.len(), 4);
        let bytes = unsafe { ptr::read_unaligned(bytes.as_ptr().cast()) };
        let mut x: u64 = i32::from_ne_bytes(bytes) as _;
        // from https://github.com/skeeto/hash-prospector
        x ^= x >> 16;
        x = x.wrapping_mul(0x21f0aaad);
        x ^= x >> 15;
        x = x.wrapping_mul(0xd35a2d97);
        x ^= x >> 16;
        self.0 = x;
    }

    // all just write the size of the array is 2 so who cares
    fn write_usize(&mut self, _: usize) {}
}

fn gen(input: &[u8]) -> impl Iterator<Item=Input> + '_ {
    input.lines()
        .map(|l| {
            let [d, b' ', rest @ ..] = l else { unreachable!(); };
            let d = match *d {
                b'U' => i16x2::from([0, 1]),
                b'D' => i16x2::from([0, -1]),
                b'L' => i16x2::from([-1, 0]),
                b'R' => i16x2::from([1, 0]),
                _ => unreachable!(),
            };
            (d, rest.parse_number().unwrap())
        })
}

fn pull(diff: Point) -> bool {
    (diff.abs().simd_eq(i16x2::splat(2))).any()
}

fn solve(input: &[u8]) -> (usize, usize) {
    let mut ks = [i16x2::splat(0); 10];
    let mut visited1 = [ks[1]].into_iter().collect::<HashSet<_, BuildHasherDefault<NullHasher>>>();
    let mut visited9 = [ks[9]].into_iter().collect::<HashSet<_, BuildHasherDefault<NullHasher>>>();

    for (delta, n) in gen(input) {
        for _ in 0..n {
            ks[0] += delta;
            for i in 0..ks.len() - 1 {
                let h = ks[i];
                let t = &mut ks[i + 1];
                let diff = h - *t;
                if pull(diff) {
                    let mv = diff.simd_clamp(i16x2::splat(-1), i16x2::splat(1));
                    *t += mv;
                }
            }
            visited1.insert(ks[1]);
            visited9.insert(ks[9]);
        }
    }

    (&visited1, &visited9).tuple_map(HashSet::len)
}

#[aoc(day9, part1)]
fn part1(input: &[u8]) -> usize {
    // let x = gen(input)
    //     .filter(|(dir, _)| *dir == i16x2::from([1, 0]))
    //     .map(|(_, n)| n as usize)
    //     .sum::<usize>();
    // println!("x = {:?}", x);

    solve(input).0
    // let mut h = i16x2::splat(0);
    // let mut t = i16x2::splat(0);
    // // let mut visited = [[false; 600]; 600];
    // let mut visited = [t].into_iter().collect::<HashSet<_, BuildHasherDefault<NullHasher>>>();
    //
    // for (delta, n) in gen(input) {
    //     for _ in 0..n {
    //         h += delta;
    //         let diff = h - t;
    //         if pull(diff) {
    //             let mv = diff.simd_clamp(i16x2::splat(-1), i16x2::splat(1));
    //             t += mv;
    //         }
    //         // visited[(t[0] + 300) as usize][(t[1] + 300) as usize] = true;
    //         visited.insert(t);
    //     }
    // }
    //
    // visited.len()
    // // visited.iter()
    // //     .flat_map(|row| row.iter())
    // //     .map(|b| *b as usize)
    // //     .sum()
}

#[aoc(day9, part2)]
fn part2(input: &[u8]) -> usize {
    solve(input).1
    // let mut ks = [i16x2::splat(0); 10];
    // let mut visited = [[false; 600]; 600];
    // let mut visited = [ks[9]].into_iter().collect::<HashSet<_, BuildHasherDefault<NullHasher>>>();
    //
    // for (delta, n) in gen(input) {
    //     for _ in 0..n {
    //         ks[0] += delta;
    //         for i in 0..ks.len() - 1 {
    //             let h = ks[i];
    //             let t = &mut ks[i + 1];
    //             let diff = h - *t;
    //             if pull(diff) {
    //                 let mv = diff.simd_clamp(i16x2::splat(-1), i16x2::splat(1));
    //                 *t += mv;
    //             }
    //         }
    //         visited[(ks[9][0] + 300) as usize][(ks[9][1] + 300) as usize] = true;
            // visited.insert(ks[9]);
        // }
    // }
    //
    // visited.len()
    // visited.iter()
    //     .flat_map(|row| row.iter())
    //     .map(|b| *b as usize)
    //     .sum()
}
