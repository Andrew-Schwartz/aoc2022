use std::ops::ControlFlow;

use aoc_runner_derive::aoc;

type Input = [u8];

const LEN: usize = 99;
const INPUT: &Input = include_bytes!("../input/2022/day8.txt");

fn gen() -> &'static [[u8; LEN + 1]] {
    let (lines, remainder) = INPUT.as_chunks::<{ LEN + 1 }>();
    assert!(remainder.is_empty());
    lines
}

#[aoc(day8, part1)]
fn part1(_input: &Input) -> usize {
    let lines = gen();

    // borders
    let mut visible = 4 * LEN - 4;

    // for loops are apparantly faster than `.filter().count()`
    for (row, line) in lines.iter().enumerate().skip(1).take(LEN - 2) {
        for (col, h) in line.iter().copied().enumerate().skip(1).take(LEN - 2) {
            let shorter = |oh: &u8| *oh < h;
            let left = line[..col].iter().all(shorter);
            if left {
                visible += 1;
                continue
            }
            let right = line[col + 1..].iter().all(shorter);
            if right {
                visible += 1;
                continue
            }
            let up = (0..row).map(|r| &lines[r][col]).all(shorter);
            if up {
                visible += 1;
                continue
            }
            let down = (row + 1..LEN).map(|r| &lines[r][col]).all(shorter);
            visible += down as usize;
        }
    }

    visible
}

#[aoc(day8, part2)]
fn part2(_input: &Input) -> usize {
    let lines = gen();

    let mut max = 0;
    for (row, line) in lines.iter().enumerate().skip(1).take(LEN - 2) {
        for (col, h) in line.iter().copied().enumerate().skip(1).take(LEN - 2) {
            fn look<I: Iterator<Item=&'static u8>>(mut i: I, h: u8) -> usize {
                let (ControlFlow::Continue(c) | ControlFlow::Break(c)) = i.try_fold(0, |count, &oh| if oh < h {
                    ControlFlow::Continue(count + 1)
                } else {
                    ControlFlow::Break(count + 1)
                });
                c
            }
            let left = look(line[..col].iter().rev(), h);
            if left == 0 { continue }
            let right = look(line[col + 1..LEN].iter(), h);
            if right == 0 { continue }
            let up = look((0..row).rev().map(|r| &lines[r][col]), h);
            if up == 0 { continue }
            let down = look((row + 1..LEN).map(|r| &lines[r][col]), h);
            if down == 0 { continue }
            let visible = left * right * up * down;
            if visible > max { max = visible; }
        }
    }

    max
}
