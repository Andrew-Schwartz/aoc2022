use aoc_runner_derive::aoc;

use crate::utils::{ByteStringExt, ParseNumber, SliceSplitting};

/*
            alt((
                tuple((
                    tag("cd "),
                    // todo not just a char?
                    take_until("\n"),
                )).map(|(_cd, dir)| File("cd".into(), FilePart::Dir(vec![]))),
                tuple((
                    tag("ls"),
                    newline,
                    separated_list0(
                        newline,
                        alt((
                            tuple((
                                number,
                                char(' '),
                                take_until("\n"),
                            )),
                            tuple((
                                tag("dir "),
                                take_until("\n"),
                            ))
                        )),
                    )
                )).map(|(_ls, _, files)| File)
            ))
 */

#[derive(Debug)]
struct Dir<'a>(&'a [u8], Vec<Dir<'a>>, Vec<File<'a>>);

// impl<'a> Display for Dir<'a> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         struct DisplayDir<'a, 'b>(usize, &'b Dir<'a>);
//
//         impl<'a> Display for DisplayDir<'a, '_> {
//             fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//                 let spaces = "  ".repeat(self.0);
//                 writeln!(f, "{spaces}- {} (dir)", self.1.0)?;
//                 for sub_dir in &self.1.1 {
//                     write!(f, "{}", DisplayDir(self.0 + 1, sub_dir))?;
//                 }
//                 for file in &self.1.2 {
//                     writeln!(f, "  {spaces}{file}")?;
//                 }
//                 Ok(())
//             }
//         }
//
//         DisplayDir(0, self).fmt(f)
//     }
// }

impl<'a> Dir<'a> {
    fn new(name: &'a Input) -> Self {
        Self(name, vec![], vec![])
    }
}

#[derive(Debug)]
struct File<'a>(&'a Input, usize);

// impl<'a> Display for File<'a> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "- {} (file, size={})", self.0, self.1)
//     }
// }

impl<'a> File<'a> {
    fn new(name: &'a Input, bytes: &Input) -> Self {
        Self(name, bytes.parse_number().unwrap())
    }
}

impl<'a> Dir<'a> {
    fn add_file(&mut self, path: &[&'a Input], name: &'a Input, bytes: &'a Input) {
        if self.0 == path[0] && path.len() == 1 || path.is_empty() {
            let file = File::new(name, bytes);
            self.2.push(file);
        } else {
            self.1.iter_mut()
                .find(|dir| dir.0 == path[1])
                .unwrap()
                .add_file(&path[1..], name, bytes);
        }
    }

    fn add_dir(&mut self, path: &[&'a Input], name: &'a Input) {
        if self.0 == path[0] && path.len() == 1 || path.is_empty() {
            let dir = Dir::new(name);
            self.1.push(dir);
        } else {
            self.1.iter_mut()
                .find(|dir| dir.0 == path[1])
                .unwrap()
                .add_dir(&path[1..], name);
        }
    }
}

type Input = [u8];

fn gen(input: &Input) -> (usize, Dir<'_>) {
    let mut count = 1;
    let dir = input.split(|&c| c == b'$')
        .skip(2)
        .fold(
            (vec![&b"/"[..]], Dir::new(b"/")),
            |(mut path, mut files), part| {
                let (cmd, rest) = part.split_at(3);
                match cmd {
                    b" cd" => {
                        match rest.trim() {
                            b".." => { path.pop(); }
                            cd => path.push(cd),
                        }
                    }
                    b" ls" => {
                        for line in rest.trim().lines() {
                            match line.split_once(b" ").unwrap() {
                                (b"dir", name) => {
                                    count += 1;
                                    files.add_dir(&path, name);
                                }
                                (bytes, name) => files.add_file(&path, name, bytes),
                            };
                        }
                    }
                    bad => unreachable!("{:?}", bad)
                };
                (path, files)
            }).1;
    (count, dir)
}

#[aoc(day7, part1)]
fn part1(input: &Input) -> usize {
    const CUTOFF: usize = 100_000;

    fn size(dir: &Dir, total: &mut usize) -> usize {
        let mut this_size = dir.2.iter().map(|f| f.1).sum::<usize>();
        for sub_dir in &dir.1 {
            this_size += size(sub_dir, total);
        }
        #[allow(clippy::cast_lossless)]
        {
            *total += this_size * (this_size <= CUTOFF) as usize;
        }
        this_size
    }
    let dir = gen(input).1;
    let mut total_size = 0;
    size(&dir, &mut total_size);
    total_size
}

#[aoc(day7, part2)]
fn part2(input: &Input) -> usize {
    const TOTAL: usize = 70_000_000;
    const FREE: usize = 30_000_000;

    // each slot is vec of `10^i..10^(i+1)`
    fn size(dir: &Dir, sizes: &mut Vec<usize>) -> usize {
        let mut total_size = dir.2.iter().map(|f| f.1).sum::<usize>();
        for sub_dir in &dir.1 {
            total_size += size(sub_dir, sizes);
        }
        sizes.push(total_size);
        total_size
    }

    let (n_dirs, dir) = gen(input);

    let mut sizes = Vec::with_capacity(n_dirs);
    let used = size(&dir, &mut sizes);
    let to_free = FREE - (TOTAL - used);

    sizes.into_iter()
        .filter(|&size| size >= to_free)
        .min()
        .unwrap()
}