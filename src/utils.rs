use std::array;
use std::ops::{Add, Mul};
use std::slice::Split;
use std::str::FromStr;

use arrayvec::ArrayVec;
use itertools::Itertools;
use nom::{AsChar, InputIter, InputTakeAtPosition, IResult, Parser};
use nom::character::complete::digit1;
use nom::combinator::map_opt;

pub trait CollectArray<T> {
    fn collect_array<const N: usize>(self) -> Result<[T; N], Result<Vec<T>, array::IntoIter<T, N>>>;
}

impl<T, I: Iterator<Item=T>> CollectArray<T> for I {
    fn collect_array<const N: usize>(mut self) -> Result<[T; N], Result<Vec<T>, array::IntoIter<T, N>>> {
        match self.next_chunk() {
            Ok(arr) => {
                let rest = self.collect_vec();
                if rest.is_empty() {
                    Ok(arr)
                } else {
                    Err(Ok(rest))
                }
            }
            Err(iter) => Err(Err(iter))
        }
    }
}

pub trait TupleIter<T> {
    const N: usize;

    fn tuple_iter(self) -> array::IntoIter<T, { Self::N }>;

    type MappedTuple<R>;

    fn tuple_map<R, F: FnMut(T) -> R>(self, f: F) -> Self::MappedTuple<R>;
}

macro_rules! tuple_iter {
    (map $idx:tt $to:tt) => {
        $to
    };
    (
        $($idx:tt)*
    ) => {
        impl<T> TupleIter<T> for ( $(tuple_iter!(map $idx T), )* ) {
            const N: usize = 0 $(+ tuple_iter!(map $idx 1))*;

            fn tuple_iter(self) -> array::IntoIter<T, { Self::N }> {
                [$(
                    self.$idx,
                )*].into_iter()
            }

            type MappedTuple<R> = ( $(tuple_iter!(map $idx R), )* );

            fn tuple_map<R, F: FnMut(T) -> R>(self, mut f: F) -> Self::MappedTuple<R> {
                (
                    $(
                        f(self.$idx),
                    )*
                )
            }
        }
    };
}

// tuple_iter!();
tuple_iter!(0);
tuple_iter!(0 1);
tuple_iter!(0 1 2);
tuple_iter!(0 1 2 3);
tuple_iter!(0 1 2 3 4);
tuple_iter!(0 1 2 3 4 5);
tuple_iter!(0 1 2 3 4 5 6);
tuple_iter!(0 1 2 3 4 5 6 7);
tuple_iter!(0 1 2 3 4 5 6 7 8);
tuple_iter!(0 1 2 3 4 5 6 7 8 9);

pub trait Number: FromStr + From<u8> + Add<Output=Self> + Mul<Output=Self> {
    const ZERO: Self;
    const TEN: Self;
    fn pow(self, exp: u32) -> Self;
}

macro_rules! number {
    ($($ty:ty)+) => {
        $(
            impl Number for $ty {
                const ZERO: Self = 0;
                const TEN: Self = 10;
                fn pow(self, exp: u32) -> Self { self.pow(exp) }
            }
        )*
    };
}

number! {
    u8 u16 u32 u64 u128 usize
    // i8 i16 i32 i64 i128 isize
}

pub trait ParseNumber {
    fn parse_number<N: Number>(self) -> Option<N>;
}

impl<'a> ParseNumber for &'a str {
    fn parse_number<N: Number>(self) -> Option<N> {
        self.parse().ok()
    }
}

impl<'a> ParseNumber for &'a [u8] {
    fn parse_number<N: Number>(self) -> Option<N> {
        self.iter()
            .rev()
            .enumerate()
            .try_fold(N::ZERO, |n, (exp, &char)| {
                (b'0' <= char && char <= b'9')
                    .then(|| n + N::from(char - b'0') * N::TEN.pow(exp as _))
            })
    }
}

pub fn number<I, N>(input: I) -> IResult<I, N>
    where I: ParseNumber + InputTakeAtPosition + InputIter + Clone,
          <I as InputIter>::Item: AsChar,
          <I as InputTakeAtPosition>::Item: AsChar,
          N: Number,
{
    map_opt(digit1, I::parse_number::<N>).parse(input)
}

pub trait TryRemove<T> {
    type Index;

    fn try_remove(&mut self, index: Self::Index) -> Option<T>;
}

impl<T> TryRemove<T> for Vec<T> {
    type Index = usize;

    fn try_remove(&mut self, index: Self::Index) -> Option<T> {
        (index < self.len()).then(|| self.remove(index))
    }
}

impl<T, const CAP: usize> TryRemove<T> for ArrayVec<T, CAP> {
    type Index = usize;

    fn try_remove(&mut self, index: Self::Index) -> Option<T> {
        (index < self.len()).then(|| self.remove(index))
    }
}

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

pub trait ByteLines<'a>: 'a {
    fn lines(self) -> Split<'a, u8, NewLine>;
}

impl<'a> ByteLines<'a> for &'a [u8] {
    fn lines(self) -> Split<'a, u8, NewLine> {
        self.split(NewLine)
    }
}

pub trait SplitSliceOnce<'a, T>: 'a {
    fn split_once<const N: usize>(self, pattern: &[T; N]) -> Option<(&'a [T], &'a [T])>;
    fn rsplit_once<const N: usize>(self, pattern: &[T; N]) -> Option<(&'a [T], &'a [T])>;
}

impl<'a, T: PartialEq> SplitSliceOnce<'a, T> for &'a [T] {
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