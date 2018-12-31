extern crate num;
use num::{BigInt, Integer, FromPrimitive, PrimInt};

mod numbers;
mod place;
mod block;
mod parse;

use numbers::{KoreanInteger, NumberSystem};

/// Parses an int into a Hangeul String.
///
/// ```md
/// Args:
/// *input* - Any {integer}.
/// *is_sino* - true  => parse as a Sino-Korean number.
///             false => parse as a Pure-Korean number.
/// ```
pub fn hangeul_from_int<T>(input: T, number_system: NumberSystem) -> String
    where T: Integer + ToString + FromPrimitive
{
    KoreanInteger::from_int(input, number_system).get_hangeul()
}

/// Parses an int String into a Hangeul String.
///
/// ```md
/// Args:
/// *input* - A String that can be parsed to an {integer}.
/// *is_sino* - true  => parse as a Sino-Korean number.
///             false => parse as a Pure-Korean number.
/// ```
pub fn hangeul_from_string(input: String, number_system: NumberSystem) -> String {
    hangeul_from_int(input.parse::<u128>().unwrap(), number_system)
}

/// Parses a BigInt into a Hangeul String.
///
/// ```md
/// Args:
/// *input* - A BigInt.
/// ```
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
