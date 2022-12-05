use std::array;

use itertools::Itertools;

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