use std::fmt::Debug;
use std::ops::{Add, Mul};
use std::str::FromStr;
use nom::{AsChar, InputIter, InputTakeAtPosition, IResult, Parser};
use nom::character::complete::digit1;
use nom::combinator::map_opt;

// pub fn take_until0<T, I, E: ParseError<I>>(
//     tag: T
// ) -> impl Fn(I) -> IResult<I, I, E>
//     where
//         I: InputTake + FindSubstring<T> + InputLength,
//         T: InputLength + Clone,
// {
//     move |i| {
//         let t = tag.clone();
//         let idx = i.find_substring(t)
//             .unwrap_or_else(|| i.input_len());
//         Ok(i.take_split(idx))
//     }
// }

pub trait Number: FromStr + From<u8> + TryFrom<u32> + Add<Output=Self> + Mul<Output=Self> {
    const ZERO: Self;
    const TEN: Self;
}

macro_rules! number {
    ($($ty:ty)+) => {
        $(
            impl Number for $ty {
                const ZERO: Self = 0;
                const TEN: Self = 10;
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
        if cfg!(debug_assertions) {
            if !self.iter().all(|c| (b'0'..=b'9').contains(c)) {
                return None;
            }
        }
        let n = self.iter()
            .fold(N::ZERO, |n, &char| n * N::TEN + N::from(char - b'0'));
        Some(n)
    }
}

pub fn number<I, N>(input: I) -> IResult<I, N>
    where I: ParseNumber + InputTakeAtPosition + InputIter + Clone,
          <I as InputIter>::Item: AsChar,
          <I as InputTakeAtPosition>::Item: AsChar,
          N: Number,
          <N as TryFrom<u32>>::Error: Debug,
{
    map_opt(digit1, I::parse_number::<N>).parse(input)
}