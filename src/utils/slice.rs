use std::mem;

pub struct NewLine;

impl FnOnce<(&u8, )> for NewLine {
    type Output = bool;

    extern "rust-call" fn call_once(self, _: (&u8, )) -> Self::Output {
        unreachable!()
    }
}

impl FnMut<(&u8, )> for NewLine {
    extern "rust-call" fn call_mut(&mut self, (&char, ): (&u8, )) -> Self::Output {
        char == b'\n'
    }
}

pub trait ByteStringExt {
    fn lines(&self) -> std::slice::Split<u8, NewLine>;

    fn trim(&self) -> &Self;
}

impl ByteStringExt for [u8] {
    fn lines(&self) -> std::slice::Split<u8, NewLine> {
        self.split(NewLine)
    }

    fn trim(&self) -> &Self {
        fn count_whitespace<'a, I: Iterator<Item=&'a u8>>(i: I) -> usize {
            i.take_while(|c| c.is_ascii_whitespace())
                .count()
        }

        let start = count_whitespace(self.iter());
        let end = count_whitespace(self.iter().rev());
        &self[start..self.len() - end]
    }
}

pub trait SliceSplitting<'a, T>: 'a {
    fn splits<'p, const N: usize>(self, pattern: &'p [T; N]) -> SliceSplit<'a, 'p, T, N>;

    fn split_once<const N: usize>(self, pattern: &[T; N]) -> Option<(&'a [T], &'a [T])>;
    fn rsplit_once<const N: usize>(self, pattern: &[T; N]) -> Option<(&'a [T], &'a [T])>;
}

impl<'a, T: PartialEq> SliceSplitting<'a, T> for &'a [T] {
    fn splits<'p, const N: usize>(self, pattern: &'p [T; N]) -> SliceSplit<'a, 'p, T, N> {
        SliceSplit {
            slice: self,
            pattern,
        }
    }

    fn split_once<const N: usize>(self, pattern: &[T; N]) -> Option<(&'a [T], &'a [T])> {
        self.array_windows()
            .position(|window| window == pattern)
            .map(|mid| {
                let (a, b) = self.split_at(mid);
                (a, &b[N..])
            })
    }

    fn rsplit_once<const N: usize>(self, pattern: &[T; N]) -> Option<(&'a [T], &'a [T])> {
        self.array_windows()
            .rposition(|window| window == pattern)
            .map(|mid| {
                let (a, b) = self.split_at(mid);
                (a, &b[N..])
            })
    }
}

pub struct SliceSplit<'s, 'p, T, const N: usize> {
    slice: &'s [T],
    pattern: &'p [T; N],
}

impl<'s, 'p, T: PartialEq, const N: usize> Iterator for SliceSplit<'s, 'p, T, N> {
    type Item = &'s [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            None
        } else if let Some((a, rest)) = self.slice.split_once(self.pattern) {
            self.slice = rest;
            Some(a)
        } else {
            Some(mem::take(&mut self.slice))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.slice.is_empty() {
            (0, Some(0))
        } else {
            // If the predicate doesn't match anything, we yield one slice.
            // If it matches every element, we yield `len() / N + 1` empty slices.
            (1, Some(self.slice.len() / N + 1))
        }
    }
}