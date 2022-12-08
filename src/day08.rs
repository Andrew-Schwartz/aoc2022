use std::ops::ControlFlow;

use aoc_runner_derive::aoc;

use crate::utils::TupleGet;

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

    for (row, line) in lines.into_iter().enumerate().skip(1).take(LEN - 2) {
        let new = line.into_iter()
            .copied()
            .enumerate()
            .skip(1).take(LEN - 2)
            .filter(|&(col, h)| {
                let shorter = |oh: &u8| *oh < h;
                let left = line[..col].into_iter().all(shorter);
                let right = line[col + 1..].into_iter().all(shorter);
                let up = (0..row).map(|r| &lines[r][col]).all(shorter);
                let down = (row + 1..LEN).map(|r| &lines[r][col]).all(shorter);
                left || right || up || down
            }).count();
        visible += new;
    }

    visible
}

#[aoc(day8, part2)]
fn part2(_input: &Input) -> usize {
    let lines = gen();

    let mut max = 0;
    for (row, line) in lines.into_iter().enumerate().skip(1).take(LEN - 2) {
        let new = line.into_iter()
            .copied()
            .enumerate()
            .skip(1).take(LEN - 2)
            .map(|(col, h)| {
                fn look<I: Iterator<Item=&'static u8>>(mut i: I, h: u8) -> usize {
                    // i.position(|oh| *oh >= h).map_or_else(|| i.count(), |c| c + 1)
                    let (ControlFlow::Continue(c) | ControlFlow::Break(c)) = i.try_fold(0, |count, &oh| if oh < h {
                        ControlFlow::Continue(count + 1)
                    } else {
                        ControlFlow::Break(count + 1)
                    });
                    c
                }
                let left = look(line[..col].into_iter().rev(), h);
                let right = look(line[col + 1..LEN].into_iter(), h);
                let up = look((0..row).rev().map(|r| &lines[r][col]), h);
                let down = look((row + 1..LEN).map(|r| &lines[r][col]), h);
                // println!("left = {:?}", left);
                left * right * up * down
            }).max()
            .unwrap();
        // println!("new = {:?}", new);
        if new > max { max = new; }
    }

    max
}
