// generic_const_exprs
#![allow(incomplete_features)]

#![feature(iter_next_chunk)]
#![feature(generic_const_exprs)]
#![feature(iter_array_chunks)]
#![feature(type_alias_impl_trait)]

#![warn(clippy::pedantic)]

mod utils;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

aoc_runner_derive::aoc_lib! { year = 2022 }