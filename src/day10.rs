use aoc_runner_derive::aoc;
use arrayvec::ArrayString;

use crate::utils::{ByteStringExt, ParseNumber};

type Input = [u8];

// #[aoc_generator(day10)]
fn gen(input: &Input) -> impl Iterator<Item=Option<i32>> + '_ {
    input.lines()
        .map(|l| match l {
            [b'n', ..] => None,
            [b'a', b'd', b'd', b'x', b' ', n @ ..] => {
                Some(n.parse_number().unwrap())
            }
            bad => unreachable!("{:?}", bad),
        })
}

#[aoc(day10, part1)]
fn part1(input: &Input) -> i32 {
    let mut signal = 0;
    let mut cycle = 0;
    let mut x = 1;

    for op in gen(input) {
        match op {
            None => {
                cycle += 1;
                if (cycle - 20) % 40 == 0 {
                    signal += cycle * x;
                }
            }
            Some(v) => {
                for _ in 0..2 {
                    cycle += 1;
                    if (cycle - 20) % 40 == 0 {
                        signal += cycle * x;
                    }
                }
                x += v;
            }
        }
    }

    signal
}

const SIZE: usize = 41 * 6;

#[aoc(day10, part2)]
fn part2(input: &Input) -> ArrayString<SIZE> {
    let mut cycle = 0;
    let mut sprite = 1;
    let mut img = ArrayString::<SIZE>::new();

    for op in gen(input) {
        match op {
            None => {
                cycle += 1;
                set_pixel(cycle, sprite, &mut img);
            }
            Some(v) => {
                for _ in 0..2 {
                    cycle += 1;
                    set_pixel(cycle, sprite, &mut img);
                }
                sprite += v;
            }
        }
    }

    img
}

fn set_pixel(cycle: i32, sprite: i32, img: &mut ArrayString<SIZE>) {
    let pos = (cycle - 1) % 40;
    if pos == 0 { img.push('\n') }
    let pixel = if (sprite - 1..=sprite + 1).contains(&pos) {
        '#'
    } else {
        ' '
    };
    img.push(pixel);
}
