use std::array;

use arrayvec::ArrayVec;
use itertools::Itertools;

pub use parsing::*;
pub use slice::*;
pub use tuple::*;

mod parsing;
mod tuple;
mod slice;

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

// pub fn separated_iter0<I, O, O2, F, G>(
//     input: I,
//     sep: G,
//     f: F,
// ) -> SeparatedIter0<I, O, O2, F, G> {
//     SeparatedIter0 {
//         input,
//         sep,
//         f,
//         state: Default::default(),
//         types: Default::default(),
//     }
// }
//
// pub struct SeparatedIter0<I, O, O2, F, G> {
//     input: I,
//     sep: G,
//     f: F,
//     state: State<I>,
//     types: PhantomData<(O, O2)>,
// }
//
// // impl<I, O, O2, F, G> SeparatedIter0<I, O, O2, F, G> {
// //     pub fn finish(self) -> IResult<I, ()> {
// //         match self.state {
// //             State::StartUnparsed
// //             | State::RunningSeparated
// //             | State::Done => Ok((self.input, ())),
// //             State::Failure(e) => Err(nom::Err::Failure(e)),
// //             State::Incomplete(needed) => Err(nom::Err::Incomplete(needed)),
// //         }
// //     }
// // }
//
// #[derive(Default)]
// enum State<I> {
//     #[default]
//     StartUnparsed,
//     RunningSeparated,
//     Done,
//     Failure(nom::error::Error<I>),
//     Incomplete(Needed),
// }
//
// impl<I, O, O2, F, G> Iterator for SeparatedIter0<I, O, O2, F, G>
//     where F: Parser<I, O, nom::error::Error<I>>,
//           G: Parser<I, O2, nom::error::Error<I>>,
//           I: Clone + InputLength + Debug,
// {
//     type Item = O;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.state {
//             State::StartUnparsed => {
//                 match self.f.parse(self.input.clone()) {
//                     Ok((i1, o)) => {
//                         self.state = State::RunningSeparated;
//                         self.input = i1;
//                         Some(o)
//                     }
//                     Err(nom::Err::Error(_)) => {
//                         self.state = State::Done;
//                         None
//                     }
//                     Err(nom::Err::Failure(e)) => {
//                         self.state = State::Failure(e);
//                         None
//                     }
//                     Err(nom::Err::Incomplete(needed)) => {
//                         println!("SU needed = {:?}", needed);
//                         self.state = State::Incomplete(needed);
//                         None
//                     }
//                 }
//             }
//             State::RunningSeparated => {
//                 let len = self.input.input_len();
//                 match self.sep.parse(self.input.clone()) {
//                     Ok((i1, _o)) => {
//                         // infinite loop check: the parser must always consume
//                         if i1.input_len() == len {
//                             self.state = State::Failure(Error::from_error_kind(i1, ErrorKind::SeparatedList));
//                             None
//                         } else {
//                             match self.f.parse(i1.clone()) {
//                                 Ok((i2, o)) => {
//                                     self.input = i2;
//                                     Some(o)
//                                 }
//                                 Err(nom::Err::Error(_)) => {
//                                     self.state = State::Done;
//                                     None
//                                 }
//                                 Err(nom::Err::Failure(e)) => {
//                                     self.state = State::Failure(e);
//                                     None
//                                 }
//                                 Err(nom::Err::Incomplete(needed)) => {
//                                     println!("RS.parse needed = {:?}", needed);
//                                     self.state = State::Incomplete(needed);
//                                     None
//                                 }
//                             }
//                         }
//                     }
//                     Err(nom::Err::Error(_)) => {
//                         self.state = State::Done;
//                         None
//                     }
//                     Err(nom::Err::Failure(e)) => {
//                         self.state = State::Failure(e);
//                         None
//                     }
//                     Err(nom::Err::Incomplete(_needed)) => {
//                         // println!("RS needed = {:?}", needed);
//                         self.state = State::Done;
//                         None
//                     }
//                 }
//             }
//             State::Done | State::Failure(_) | State::Incomplete(_) => None
//         }
//     }
// }