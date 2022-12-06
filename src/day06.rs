use aoc_runner_derive::aoc;

fn unique<const N: usize>(arr: &[u8; N]) -> bool {
    let mut bits = 0_u32;
    for &char in arr {
        let bit = 1 << (char - b'a');
        if bits & bit != 0 { return false; }
        bits |= bit
    }
    true
}

fn solve<const N: usize>(input: &[u8]) -> usize {
    input.array_windows::<N>()
        .position(unique)
        .unwrap() + N
}

#[aoc(day6, part1)]
fn part1(input: &[u8]) -> usize {
    solve::<4>(input)
}

#[aoc(day6, part2)]
fn part2(input: &[u8]) -> usize {
    solve::<14>(input)
}

const fn const_solve<const N: usize>() -> usize {
    let input = include_bytes!("../input/2022/day6.txt");

}