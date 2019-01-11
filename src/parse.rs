use numbers;
use block;
use place;

use std::cmp::min;
use math::{Sign};

pub fn parse_hangeul_sino(numbers: Vec<char>) -> String {
    let len = numbers.len() - 1;
    let mut output = String::new();
    let mut iter = numbers.iter().enumerate().peekable();

    while let Some((idx, input_num)) = iter.next() {
        if let Some(sign) = Sign::from_char(&input_num) {
            if idx != len { panic!("+ or - symbol isn't at the beginning of the input."); }
            output.push_str(&sign.to_string_rev());
            continue;
        }

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
            zeroes += idx;

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

pub fn parse_hangeul_pure(numbers: Vec<char>) -> String {
    let mut output = String::new();
    let mut iter = numbers.iter().enumerate().peekable();

    while let Some((idx, input_num)) = iter.next() {
        match (idx, input_num) {
            (0, '0') => {
                if let Some((_, next_num)) = iter.peek() {
                    let new_input = format!("{}{}", next_num, "0");
                    println!("{}", new_input);
                    let num = numbers::KoreanNumberPure::from_str(&new_input).unwrap();

                    output.push_str(num.to_str());
                    return output;
                } else {
                    let num = numbers::KoreanNumberPure::from_char(input_num).unwrap();
                    output.push_str(num.to_str());
                    return output;
                }
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

pub fn parse_hangeul_float(left_side: Vec<char>, right_side: Vec<char>) -> String {
    let mut output = String::new();
    let mut left_side_chars: Vec<char> = vec![];

    output
}
