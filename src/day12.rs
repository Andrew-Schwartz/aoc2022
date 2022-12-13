use std::mem::MaybeUninit;

use aoc_runner::identity;
use aoc_runner_derive::aoc;

use crate::utils::{array2_like, ByteConstExt, split_array_ref};

type Input = [u8];
type Pt = (usize, usize);

// #[aoc_generator(day12)]
// fn gen(input: &Input) -> &[u8] {
//     &input
// }

const WH: Pt = const {
    let input = include_bytes!("../input/2022/day12.txt");
    let width = input.find(b'\n').unwrap();
    let height = input.len() / width;
    (width, height)
};

const GRID: [[u8; WH.0]; WH.1] = const {
    let mut input = include_bytes!("../input/2022/day12.txt").get(..).unwrap();
    let mut idx = 0;
    let mut arr = MaybeUninit::uninit_array();
    while idx < WH.1 {
        let (row, rest) = split_array_ref::<_, { WH.0 }>(input);
        input = rest.get(1..).unwrap();
        // let start = WH.0 * idx + idx;
        // let end = WH.0 * (idx + 1) + idx;
        arr[idx].write(*row);
        idx += 1;
    }
    unsafe { MaybeUninit::array_assume_init(arr) }
};

fn get<T: Copy>(dist: &[[T; WH.0]; WH.1], pt: Pt) -> T {
    dist[pt.1][pt.0]
}

fn adj((y, x): Pt) -> impl Iterator<Item=Pt> {
    // underflows if not lazy
    #[allow(clippy::unnecessary_lazy_evaluations)]
    [
        (y != 0).then(|| (y - 1, x)),
        (x != 0).then(|| (y, x - 1)),
        (y != WH.0 - 1).then(|| (y + 1, x)),
        (x != WH.1 - 1).then(|| (y, x + 1)),
    ].into_iter().filter_map(identity)
}

#[allow(clippy::cast_possible_wrap)]
fn shortest_path(find_a: bool) -> u32 {
    let input = GRID;

    let [end, start] = [b'E', b'S'].map(|pt| input.iter()
        .enumerate()
        .find_map(|(r, &row)| row.find(pt).map(|c| (c, r)))
        .unwrap());

    let mut dist = array2_like(&GRID, 0);
    let mut active = vec![end];

    while get(&dist, start) == 0 || adj(start).any(|pt| get(&dist, pt) == 0) {
        let pt = active.remove(0);
        let height = match get(&input, pt) {
            b'Z' => b'z',
            b'S' => b'a',
            height => height,
        } as i8;
        for adj_pt in adj(pt) {
            let adj_height = get(&input, adj_pt);
            let adj_height = if adj_height == b'S' { b'a' } else { adj_height } as i8;
            if height - adj_height <= 1 {
                let adj_dist = get(&dist, adj_pt);
                if (adj_dist == 0 && adj_pt != end) || adj_dist >= get(&dist, pt) {
                    if !active.contains(&adj_pt) {
                        active.push(adj_pt);
                    }
                    let adj_score = get(&dist, pt) + 1;
                    if find_a && adj_height == b'a' as i8 { return adj_score; }
                    dist[adj_pt.1][adj_pt.0] = adj_score;
                }
            }
        }
    }

    get(&dist, start)
}

#[aoc(day12, part1)]
fn part1(_: &Input) -> u32 {
    shortest_path(false)
}

#[aoc(day12, part2)]
fn part2(_: &Input) -> u32 {
    shortest_path(true)
}
