use std::simd::{i16x2, SimdInt, SimdPartialEq};

use aoc_runner_derive::aoc;
use bitvec::BitArr;
use bitvec::array::BitArray;

use crate::utils::{SliceConstExt, ByteStringExt, parse_number, ParseNumber};

type Point = i16x2;
type Input = (Point, u8);

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

fn solve<const N: usize>(input: &[u8]) -> usize {
    #[allow(clippy::many_single_char_names)]
    const MAXES: [[i16; 2]; 2] = const {
        let mut b = include_bytes!("../input/2022/day9.txt").as_slice();
        let mut pos = [0; 2];
        let mut x = [0; 2];
        let mut y = [0; 2];
        while b.len() != 0{
            let (idx, delta) = match b.take_n(2)[0] {
                b'U' => (1, 1),
                b'D' => (1, -1),
                b'L' => (0, -1),
                b'R' => (0, 1),
                _ => (0, 0),
            };

            let n = b.get(..).unwrap().find(&b'\n').unwrap();
            let num = parse_number(b.take_n(n));

            pos[idx] += num * delta;

            if pos[0] < x[0] { x[0] = pos[0] }
            if pos[0] > x[1] { x[1] = pos[0] }
            if pos[1] < y[0] { y[0] = pos[1] }
            if pos[1] > y[1] { y[1] = pos[1] }
            b.take_n(1);
        }
        [x, y]
    };
    const X_MIN: i16 = MAXES[0][0];
    const X_MAX: i16 = MAXES[0][1];
    const SX: usize = (X_MAX - X_MIN + 1) as usize;
    const Y_MIN: i16 = MAXES[1][0];
    const Y_MAX: i16 = MAXES[1][1];
    const SY: usize = (Y_MAX - Y_MIN + 1) as usize;
    type Visited = BitArr!(for SX * SY);

    let mut ks = [i16x2::splat(0); N];

    let mut visitedb: Visited = BitArray::ZERO;

    for (delta, n) in gen(input) {
        for _ in 0..n {
            ks[0] += delta;
            for i in 0..ks.len() - 1 {
                let h = ks[i];
                let t = &mut ks[i + 1];
                let diff = h - *t;
                if diff.abs().simd_eq(i16x2::splat(2)).any() {
                    let mv = diff.signum();
                    *t += mv;
                } else {
                    break;
                }
            }
            let idx = (ks[N - 1][0] - X_MIN) as usize + SX * (ks[N - 1][1] - Y_MIN) as usize;
            *unsafe { visitedb.get_unchecked_mut(idx) } = true;
        }
    }

    visitedb.count_ones()
}

#[aoc(day9, part1)]
fn part1(input: &[u8]) -> usize {
    solve::<2>(input)
}

#[aoc(day9, part2)]
fn part2(input: &[u8]) -> usize {
    solve::<10>(input)
}
