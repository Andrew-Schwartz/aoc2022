use std::fmt::Debug;
use std::iter::Product;
use std::ops::{Add, AddAssign, Mul, Rem};
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

pub trait Number: FromStr
                + From<u8>
                + TryFrom<u32>
                + Add<Output=Self>
                + AddAssign
                + Mul<Output=Self>
                + Rem<Output=Self>
                + Ord
                + Product
                + Debug
                + Copy
{
    const SIGNED: bool;
    const ZERO: Self;
    const ONE: Self;
    const TEN: Self;
    #[must_use]
    fn neg(self) -> Self;
}

macro_rules! number {
    (false, $self:ident) => { $self };
    (true, $self:ident) => { -$self };
    ($signed:tt => $($ty:ty)+;) => {
        $(
            impl Number for $ty {
                const SIGNED: bool = $signed;
                const ZERO: Self = 0;
                const ONE: Self = 1;
                const TEN: Self = 10;
                #[inline(always)]
                fn neg(self) -> Self { number!($signed, self) }
            }
        )*
    };
}

number! { false => u8 u16 u32 u64 u128 usize; }
number! { true  => /*i8*/ i16 i32 i64 i128 isize; }

pub trait ParseNumber {
    fn parse_number<N: Number>(self) -> Option<N>;
}

impl<'a> ParseNumber for &'a str {
    fn parse_number<N: Number>(self) -> Option<N> {
        self.parse().ok()
    }
}

impl<'a> ParseNumber for &'a [u8] {
    fn parse_number<N: Number>(mut self) -> Option<N> {
        if cfg!(debug_assertions) && !self.iter().all(|c| (b'0'..=b'9').contains(c) || *c == b'-') {
            return None;
        }
        // println!("self = {:?}", self);
        let mut neg = false;
        let n = if N::SIGNED {
            match *self.take_first()? {
                b'-' => {
                    neg = true;
                    N::ZERO
                }
                digit => N::from(digit - b'0'),
            }
        } else {
            N::ZERO
        };
        // println!("n = {:?}", n);
        let n = self.iter()
            .fold(n, |n, &digit| n * N::TEN + N::from(digit - b'0'));
        if neg {
            Some(n.neg())
        } else {
            Some(n)
        }
    }
}

/// # Errors
///
/// Returns errors if the input can't be parsed as this number type
pub fn number<I, N>(input: I) -> IResult<I, N>
    where I: ParseNumber + InputTakeAtPosition + InputIter + Clone,
          <I as InputIter>::Item: AsChar,
          <I as InputTakeAtPosition>::Item: AsChar,
          N: Number,
// <N as TryFrom<u32>>::Error: Debug,
{
    map_opt(digit1, I::parse_number::<N>).parse(input)
}