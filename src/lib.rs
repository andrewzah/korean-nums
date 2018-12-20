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
    //let iter = numbers.iter().rev().peekable();

    for (idx, number) in numbers.iter().enumerate() {
        let remaining = len - idx;
        let block_len = cmp::min(remaining, 3);
        let num = Number::from_str(number).unwrap();

        if block_len != 0 {
            let block_slice = &numbers[idx+1..idx+block_len];
            if num == 0 && block_slice.iter().any(|x| x != "0") {
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
