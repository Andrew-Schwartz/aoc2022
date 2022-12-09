use std::array;

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

// pub trait TupleType<const N: usize> {
//     type Type;
//
//     fn _get(self) -> Self::Type;
// }
//
// macro_rules! tuple_type {
//     // (typed [$($ts:ty),+] $($idx:tt)+) => {
//     //
//     // };
//     // // catch infinite recursion
//     // (typed $($tt:tt)+) => {
//     //     concat!($($tt),+)
//     // };
//     // the input, first step is to make types
//     ($($t:ty)+ ; $($idx:tt)+) => {
//         // tuple_type! { typed
//         //     [$( paste::paste! { [< T $idx >] } ),+]
//         //     $($idx:tt)+
//         // }
//         $(
//             impl<$($t),+> TupleType<$idx> for ( $($t),+ ) {
//                 type Type = $t;
//
//                 fn _get(self) -> Self::Type {
//                     self.$idx
//                 }
//             }
//         )+
//     };
// }
//
// // tuple_type!(T0; 0);
//
// impl<T0> TupleType<0> for (T0, ) {
//     type Type = T0;
//
//     fn _get(self) -> Self::Type {
//         self.0
//     }
// }
//
// // impl<T0, T1> TupleType<0> for (T0, T1, ) {
// //     type Type = T0;
// //
// //     fn _get(self) -> Self::Type {
// //         self.0
// //     }
// // }
// //
// // impl<T0, T1> TupleType<1> for (T0, T1, ) {
// //     type Type = T1;
// //
// //     fn _get(self) -> Self::Type {
// //         self.1
// //     }
// // }
//
//
// /// Get element of this tuple by index.
// ///
// /// ```
// /// # use crate::aoc2022::utils::TupleGet;
// /// let t0 = 42;
// /// let tup = (t0, );
// /// assert_eq!(t0, tup.get::<0>())
// /// ```
// ///
// /// Does not compile if the index is out of bounds for this tuple.
// /// ```compile_fail
// /// # use crate::aoc2022::utils::TupleGet;
// /// let t0 = 42;
// /// let tup = (t0, );
// /// assert_eq!(t0, tup.get::<1>())
// /// ```
// pub trait TupleGet {
//     fn get<const N: usize>(self) -> <Self as TupleType<N>>::Type
//         where Self: TupleType<N>,
//     ;
// }
//
// impl<T0, > TupleGet for (T0, ) {
//     fn get<const N: usize>(self) -> <Self as TupleType<N>>::Type
//         where Self: TupleType<N>,
//     {
//         <Self as TupleType<N>>::_get(self)
//     }
// }
//
// impl<T0, T1, > TupleGet for (T0, T1, ) {
//     fn get<const N: usize>(self) -> <Self as TupleType<N>>::Type
//         where Self: TupleType<N>,
//     {
//         <Self as TupleType<N>>::_get(self)
//     }
// }