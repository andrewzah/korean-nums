use num::{BigInt, FromPrimitive, Integer};

mod block;
mod errors;
mod math;
mod numbers;
mod parse;
mod place;
mod utilities;

use crate::errors::HangeulError;
use crate::math::KoreanMathOp;
use crate::numbers::KoreanInteger;

pub use crate::numbers::NumberSystem;

/// Parses an int into a Hangeul String.
pub fn hangeul_from_int<T>(input: T, number_system: NumberSystem) -> Result<String, HangeulError>
where
    T: Copy + Integer + ToString + FromPrimitive,
{
    KoreanInteger::from_int(input, number_system).get_hangeul()
}

/// Parses a BigInt into a Hangeul String.
pub fn hangeul_from_bigint(input: BigInt) -> Result<String, HangeulError> {
    let prepared_input = input.to_string().replace(",", "").chars().rev().collect();

    parse::parse_hangeul_sino(prepared_input)
}

/// Parses a Float (or currency) into a Hangeul String.
pub fn hangeul_from_float(input: f64) -> Result<String, HangeulError> {
    let string = input.to_string();
    let numbers = string.split(".").collect::<Vec<&str>>();

    let left_side = hangeul_from_int(
        numbers[0].parse::<u128>().unwrap(),
        NumberSystem::SinoKorean,
    )?;
    let right_side = hangeul_from_int(numbers[1].parse::<u128>().unwrap(), NumberSystem::SinoKorean)?;

    Ok(format!("{} 점 {}", left_side, right_side))
}

/// Parses a valid math expression into a Hangeul String.
pub fn hangeul_from_expression(input: &str) -> Result<String, HangeulError> {
    let inputs = input.split(" ").collect::<Vec<&str>>();
    if inputs.len() != 3 {
        panic!("Wrong number of inputs supplied.")
    }

    let left_hangeul =
        KoreanInteger::from_int(inputs[0].parse::<u128>().unwrap(), NumberSystem::SinoKorean)
            .get_hangeul()?;

    let right_hangeul =
        KoreanInteger::from_int(inputs[2].parse::<u128>().unwrap(), NumberSystem::SinoKorean)
            .get_hangeul()?;

    if let Some(math_op) = KoreanMathOp::from_str(inputs[1]) {
        Ok(math_op.to_str(&left_hangeul, &right_hangeul))
    } else {
        Err(HangeulError::Generic(format!("The {} expression is not supported (yet). Put in a ticket.", inputs[2])))
    }
}
