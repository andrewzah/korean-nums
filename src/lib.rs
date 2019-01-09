extern crate num;
use num::{BigInt, Float, Integer, FromPrimitive};

mod numbers;
mod place;
mod block;
mod parse;
mod math;

use numbers::{KoreanInteger};
pub use numbers::NumberSystem;

/// Parses an int into a Hangeul String.
pub fn hangeul_from_int<T>(input: T, number_system: NumberSystem) -> String
    where T: Copy + Integer + ToString + FromPrimitive
{
    KoreanInteger::from_int(input, number_system).get_hangeul()
}

/// Parses a BigInt into a Hangeul String.
pub fn hangeul_from_bigint(input: BigInt) -> String
{
    if input < FromPrimitive::from_i8(0).unwrap() {
        panic!("Input cannot be negative.")
    }
    let prepared_input = input
        .to_string()
        .replace(",", "")
        .chars()
        .rev()
        .collect();

    parse::parse_hangeul_sino(prepared_input)
}

pub fn hangeul_from_float<F>(input: F) -> String
    where F: Copy + ToString + Float
{
    let string = input.to_string();
    let numbers = string.split(".")
        .collect::<Vec<&str>>();

    let left_side =
        KoreanInteger::from_int(
            numbers[0].parse::<u128>().unwrap(),
            NumberSystem::SinoKorean
        ).get_hangeul();
    let right_side =
        KoreanInteger::from_int(
            numbers[1].parse::<u128>().unwrap(),
            NumberSystem::SinoKorean
        ).get_hangeul();

    format!("{} 점 {}", left_side, right_side)
}
