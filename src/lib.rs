// generic_const_exprs
#![allow(incomplete_features)]

#![feature(iter_next_chunk)]
#![feature(generic_const_exprs)]
#![feature(iter_array_chunks)]
#![feature(type_alias_impl_trait)]
#![feature(array_windows)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(is_some_and)]
#![feature(portable_simd)]
#![feature(array_chunks)]
#![feature(slice_as_chunks)]
#![feature(try_blocks)]
#![feature(split_array)]

#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]

pub mod utils;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

aoc_runner_derive::aoc_lib! { year = 2022 }
