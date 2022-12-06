use aoc_runner_derive::aoc;

/*
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
*/

const fn const_solve<const N: usize>() -> usize {
    let input = include_bytes!("../input/2022/day6.txt");

    let mut i = 0;
    loop {
        let mut bits = 0_u32;

        let mut j = 0;
        while j < N {
            let bit = 1 << (input[i + j] - b'a');
            if bits & bit != 0 { break }
            bits |= bit;
            j += 1;
        }
        if j == N { return i + N }

        i += 1;
    }
}

#[aoc(day6, part1)]
fn part1(_: &[u8]) -> usize {
    const SOLUTION: usize = const_solve::<4>();
    SOLUTION
}

#[aoc(day6, part2)]
fn part2(_: &[u8]) -> usize {
    const SOLUTION: usize = const_solve::<14>();
    SOLUTION
}