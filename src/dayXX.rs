use aoc_runner_derive::aoc;

type Parsed = [u8];

// #[aoc_generator(day)]
fn gen(input: &[u8]) -> &[u8] {
    &input
}

#[aoc(day, part1)]
fn part1(input: &Parsed) -> u32 {
    let input = gen(input);

    1
}

// #[aoc(day, part2)]
// fn part2(input: &Input) -> u32 {
//     let input = gen(input);
//
//     1
// }