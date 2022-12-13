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
#![feature(array_zip)]
#![feature(array_methods)]
#![feature(const_slice_index)]
#![feature(const_option)]
#![feature(slice_take)]
#![feature(inline_const)]
#![feature(const_trait_impl)]
#![feature(const_mut_refs)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(const_maybe_uninit_uninit_array)]
#![feature(const_maybe_uninit_write)]
#![feature(const_maybe_uninit_array_assume_init)]
#![feature(const_slice_split_at_not_mut)]

#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::module_name_repetitions)]

pub mod utils;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;

aoc_runner_derive::aoc_lib! { year = 2022 }
