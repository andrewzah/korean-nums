extern crate num;

mod numbers;
mod place;
mod block;

use std::cmp::{min};
use std::cmp::Ord;
use num::{FromPrimitive, PrimInt};

pub fn hangul_from_string(input: String, is_sino: bool) -> String {
    hangul_from_int(input.parse::<u64>().unwrap(), is_sino)
}

pub fn hangul_from_int<T>(input: T, is_sino: bool) -> String
    where T: PrimInt + ToString + FromPrimitive
{
    validate_sino(input, is_sino);
    let prepared_input = prepare_input(input);
    match is_sino {
        true => parse_hangul_sino(prepared_input),
        false => parse_hangul_pure(prepared_input),
    }
}

fn validate_sino<T>(input: T, is_sino: bool)
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

fn parse_hangul_sino(numbers: Vec<char>) -> String {
    let len = numbers.len() - 1;
    let mut output = String::new();
    let mut iter = numbers.iter().enumerate();

    while let Some((idx, input_num)) = iter.next() {
        let remaining = len - idx;
        let min_peek_len = min(remaining, 3);
        let mut num = numbers::KoreanNumberSino::from_char(input_num).unwrap();

        if min_peek_len != 0 && num == 0 {
            let mut zeroes = 1;
            while let Some((_, next_num)) = iter.next() {
                if *next_num != '0' {
                    num = numbers::KoreanNumberSino::from_char(next_num).unwrap();
                    break;
                }
                zeroes += 1;
            }

            if let Some(block) = block::Block::from_usize(zeroes) {
                output.push_str(block.to_str());
                if num != 1 {
                    output.push_str(num.to_str());
                }
                continue;
            } else {
                let zmod = zeroes % 4;
                if zeroes >= 4 {
                    let block = block::Block::from_usize(zeroes - zmod).unwrap();
                    output.push_str(block.to_str());
                }
                let place = place::Place::from_usize(zeroes % 4).unwrap();
                output.push_str(place.to_str());
                if num != 1 {
                    output.push_str(num.to_str());
                }
                continue;
            }
        }

        let modulo = idx % 4;
        match modulo {
            1|2|3 => {
                let place = place::Place::from_usize(modulo).unwrap();
                output.push_str(place.to_str());

                if num != 1 {
                    output.push_str(num.to_str());
                }
            },
            _ => {
                if idx != 0 {
                    let block = block::Block::from_usize(idx)
                        .expect("Block counter doesn't go high enough for this...");
                    output.push_str(&block.to_str_with_space());
                    if num != 1 || remaining > 0 {
                        output.push_str(num.to_str());
                    }
                } else {
                    output.push_str(num.to_str());
                }
            }
        }
    }

    output.chars().rev().collect::<String>()
}

fn parse_hangul_pure(numbers: Vec<char>) -> String {
    let mut output = String::new();
    let mut iter = numbers.iter().enumerate().peekable();

    println!("\nnumbers: {:?}", numbers);
    while let Some((idx, input_num)) = iter.next() {
        match (idx, input_num) {
            (0, '0') => {
                let (_, next_num) = iter.next().unwrap();
                let new_input = format!("{}{}", next_num, "0");
                let num = numbers::KoreanNumberPure::from_str(&new_input).unwrap();

                output.push_str(num.to_str());
                return output;
            },
            (0, _) => {
                if let Some((_, next_num)) = iter.peek() {
                    let next_input = format!("{}{}", next_num, "0");
                    let next_num = numbers::KoreanNumberPure::from_str(&next_input).unwrap();
                    output.push_str(next_num.to_str());

                    let input_num = numbers::KoreanNumberPure::from_char(input_num).unwrap();
                    output.push_str(input_num.to_str());
                    return output;
                } else {
                    let num = numbers::KoreanNumberPure::from_char(input_num).unwrap();
                    output.push_str(num.to_str());
                }
            },
            (1, _) => {
                let num = numbers::KoreanNumberPure::from_char(input_num).unwrap();
                output.push_str(num.to_str());
            }
            (_, _) => {}
        }
    }
    output
}
