extern crate num;

mod errors;
mod block;
mod math;
mod numbers;
mod parse;
mod place;
mod utilities;

use num::{BigInt, Integer, FromPrimitive};
use crate::math::{KoreanMathOp};
use crate::numbers::{KoreanInteger};

pub use crate::numbers::NumberSystem;

/// Parses an int into a Hangeul String.
pub fn hangeul_from_int<T>(input: T, number_system: NumberSystem) -> String
    where T: Copy + Integer + ToString + FromPrimitive
{
    KoreanInteger::from_int(input, number_system).get_hangeul()
}

/// Parses a BigInt into a Hangeul String.
pub fn hangeul_from_bigint(input: BigInt) -> String
{
    let prepared_input = input
        .to_string()
        .replace(",", "")
        .chars()
        .rev()
        .collect();

    parse::parse_hangeul_sino(prepared_input)
}

pub fn hangeul_from_money(input: f64) -> String
{
    let string = input.to_string();
    let numbers = string.split(".").collect::<Vec<&str>>();

    let left_side =
        hangeul_from_int(
            numbers[0].parse::<u128>().unwrap(),
            NumberSystem::SinoKorean);
    let right_side =
        hangeul_from_int(
            numbers[1].parse::<i8>().unwrap(),
            NumberSystem::SinoKorean);

    format!("{} 점 {}", left_side, right_side)
}

pub fn hangeul_from_expression(input: &str) -> String {
    let inputs = input.split(" ").collect::<Vec<&str>>();
    if inputs.len() != 3 { panic!("Wrong number of inputs supplied.") }

    let left_hangeul = 
        KoreanInteger::from_int(
            inputs[0].parse::<u128>().unwrap(),
            NumberSystem::SinoKorean
        ).get_hangeul();

    let right_hangeul = 
        KoreanInteger::from_int(
            inputs[2].parse::<u128>().unwrap(),
            NumberSystem::SinoKorean
        ).get_hangeul();

    if let Some(math_op) = KoreanMathOp::from_str(inputs[1]) {
        math_op.to_str(&left_hangeul, &right_hangeul)
    }
    else {
        panic!("The {} expression is not supported (yet). Put in a ticket.", inputs[2])
    }
}
