mod number;
mod place;
mod block;

use number::*;
use place::*;
use block::*;
use std::cmp;

pub fn calculate(numbers: Vec<String>) -> String {
    let len = numbers.len() - 1;
    let mut output = String::from("");
    let mut iter = numbers.iter().enumerate();

    while let Some((idx, input_num)) = iter.next() {
        let remaining = len - idx;
        let min_peek_len = cmp::min(remaining, 3);
        let mut num = Number::from_str(input_num).unwrap();

        if min_peek_len != 0 && num == 0 {
            let mut zeroes = 1;
            while let Some((_, next_num)) = iter.next() {
                if next_num != "0" {
                    num = Number::from_str(next_num).unwrap();
                    break;
                }
                zeroes += 1;
            }

            if let Some(block) = Block::from_usize(zeroes) {
                output.push_str(block.to_str_no_space());
                if num != 1 {
                    output.push_str(num.to_str_sino());
                }
                continue;
            } else {
                let zmod = zeroes % 4;
                if zeroes >= 4 {
                    let block = Block::from_usize(zeroes - zmod).unwrap();
                    output.push_str(block.to_str_no_space());
                }
                let place = Place::from_usize(zeroes % 4).unwrap();
                output.push_str(place.to_str());
                if num != 1 {
                    output.push_str(num.to_str_sino());
                }
                continue;
            }
        }

        let modulo = idx % 4;
        match modulo {
            1|2|3 => {
                let place = Place::from_usize(modulo).unwrap();
                output.push_str(place.to_str());

                if num != 1 {
                    output.push_str(num.to_str_sino());
                }
            },
            _ => {
                if idx != 0 {
                    let block = Block::from_usize(idx)
                        .expect("Block counter doesn't go high enough for this...");
                    output.push_str(block.to_str());
                    if num != 1 || remaining > 0 {
                        output.push_str(num.to_str_sino());
                    }
                } else {
                    output.push_str(num.to_str_sino());
                }
            }
        }
    }

    output.chars().rev().collect::<String>()
}

pub fn str_to_vec_string(s: &str) -> Vec<String> {
    s
        .chars()
        .map(|c| c.to_string())
        .collect()
}

pub fn i32_to_vec_string(num: i32) -> Vec<String> {
    str_to_vec_string(&num.to_string())
}

pub fn u32_to_vec_string(num: u32) -> Vec<String> {
    str_to_vec_string(&num.to_string())
}

pub fn strip_string(s: &str) -> String {
    s.replace(",", "")
}

pub fn parse_string(s: &str) -> Vec<String> {
    let mut vec = str_to_vec_string(&strip_string(s));
    vec.reverse();
    vec
}
