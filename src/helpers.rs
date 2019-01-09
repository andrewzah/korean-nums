use num::{BigInt, FromPrimitive, PrimInt};
use parse;

/// Parses an int into a Hangeul String.
///
/// ```md
/// Args:
/// *input* - Any {integer}.
/// *is_sino* - true  => parse as a Sino-Korean number.
///             false => parse as a Pure-Korean number.
/// ```
#[allow(dead_code)]
pub fn hangeul_from_int<T>(input: T, is_sino: bool) -> String
    where T: PrimInt + ToString + FromPrimitive
{
    validate(input, is_sino);
    let prepared_input = prepare_input(input);
    match is_sino {
        true => parse::parse_hangeul_sino(prepared_input),
        false => parse::parse_hangeul_pure(prepared_input),
    }
}

/// Parses an int String into a Hangeul String.
///
/// ```md
/// Args:
/// *input* - A String that can be parsed to an {integer}.
/// *is_sino* - true  => parse as a Sino-Korean number.
///             false => parse as a Pure-Korean number.
/// ```
#[allow(dead_code)]
pub fn hangeul_from_string(input: String, is_sino: bool) -> String {
    hangeul_from_int(input.parse::<u64>().unwrap(), is_sino)
}

/// Parses a BigInt into a Hangeul String.
///
/// ```md
/// Args:
/// *input* - A BigInt.
/// ```
#[allow(dead_code)]
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

fn validate<T>(input: T, is_sino: bool)
    where T: PrimInt + ToString + FromPrimitive
{
    if is_sino == false && input > FromPrimitive::from_u64(99).unwrap() {
        panic!("Pure korean numbers only go up to 99.");
    }
    if input < FromPrimitive::from_u64(0).unwrap() {
        panic!("Input cannot be negative.")
    }
}

fn prepare_input<T>(input: T) -> Vec<char>
    where T: PrimInt + ToString + Ord
{
    let nums = input
        .to_string()
        .replace(",", "")
        .chars()
        .rev()
        .collect();

    nums
}

