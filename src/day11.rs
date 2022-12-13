use aoc_runner_derive::aoc;
use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::tuple;

use crate::utils::{number, Number};

type Input = [u8];
type Parsed<Item> = Vec<Monkey<Item>>;

#[derive(Debug, Copy, Clone)]
enum Op<Item> {
    Add(Item),
    Mul(Item),
    Sqr,
}

impl<Item: Number> Op<Item> {
    fn parse(str: &Input) -> IResult<&Input, Self> {
        alt((
            tuple((tag("+ "), number)).map(|(_, n)| Self::Add(n)),
            tuple((tag("* "), number)).map(|(_, n)| Self::Mul(n)),
            tag("* old").map(|_| Self::Sqr),
        )).parse(str)
    }
}

#[derive(Debug, Clone)]
struct Monkey<Item> {
    items: Vec<Item>,
    op: Op<Item>,
    divisible: Item,
    tf: [usize; 2],
}

impl<Item: Number> Monkey<Item> {
    fn inspect(&self, item: Item) -> Item {
        match self.op {
            Op::Add(n) => item + n,
            Op::Mul(n) => item * n,
            Op::Sqr => item * item,
        }
    }
}

impl<Item: Number> Monkey<Item> {
    fn parse(str: &Input) -> IResult<&Input, Self> {
        tuple((
            tag("Monkey "),
            digit1,
            tag(":\n  Starting items: "),
            separated_list0(tag(", "), number),
            tag("\n  Operation: new = old "),
            Op::parse,
            tag("\n  Test: divisible by "),
            number,
            tag("\n    If true: throw to monkey "),
            number,
            tag("\n    If false: throw to monkey "),
            number,
        )).map(|(_monkey, _n, _starting, items, _op, op, _test, test, _true, tr, _false, flse)| {
            Self {
                items,
                op,
                divisible: test,
                tf: [flse, tr],
            }
        }).parse(str)
    }
}

// #[aoc_generator(day11)]
fn gen<Item: Number>(input: &Input) -> Parsed<Item> {
    separated_list1(
        tag("\n\n"),
        Monkey::parse,
    ).parse(input)
        .unwrap().1
}

fn solve<Item, Count>(
    mut monkeys: Parsed<Item>,
    rounds: usize,
    f: impl Fn(Item) -> Item,
) -> Count
    where Item: Number,
          Count: Number,
{
    let mut count = vec![Count::ZERO; monkeys.len()];

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop() {
                let monkey = &monkeys[i];
                count[i] += Count::ONE;
                let item = monkey.inspect(item);
                let item = f(item);
                let idx = item % monkey.divisible == Item::ZERO;
                let throw_to = monkey.tf[idx as usize];
                monkeys[throw_to].items.push(item);
            }
        }
    }

    count.sort_unstable();
    count.into_iter().rev().take(2).product()
}

#[aoc(day11, part1)]
fn part1(input: &[u8]) -> u32 {
    let monkeys = gen::<u32>(input);
    let solved = solve(monkeys, 20, |item| item / 3);
    solved
}

#[aoc(day11, part2)]
fn part2(input: &[u8]) -> u64 {
    let monkeys = gen::<u64>(input);
    let lcm = monkeys.iter().map(|m| m.divisible).product::<u64>();
    solve(monkeys, 10_000, |item| item % lcm)
}
