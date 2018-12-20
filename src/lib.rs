mod numbers;
mod place;
mod block;

use numbers::*;
use place::*;
use block::*;
use std::cmp;

pub fn str_to_hangul(input: &str) -> String {
    let nums: Vec<char> = input
        .replace(",", "")
        .chars()
        .rev()
        .collect();

    calculate(nums)
}

pub fn calculate(numbers: Vec<char>) -> String {
    let len = numbers.len() - 1;
    let mut output = String::from("");
    let mut iter = numbers.iter().enumerate();

    while let Some((idx, input_num)) = iter.next() {
        let remaining = len - idx;
        let min_peek_len = cmp::min(remaining, 3);
        let mut num = SinoNumber::from_char(input_num).unwrap();

        if min_peek_len != 0 && num == 0 {
            let mut zeroes = 1;
            while let Some((_, next_num)) = iter.next() {
                if *next_num != '0' {
                    num = SinoNumber::from_char(next_num).unwrap();
                    break;
                }
                zeroes += 1;
            }

            if let Some(block) = Block::from_usize(zeroes) {
                output.push_str(block.to_str());
                if num != 1 {
                    output.push_str(num.to_str());
                }
                continue;
            } else {
                let zmod = zeroes % 4;
                if zeroes >= 4 {
                    let block = Block::from_usize(zeroes - zmod).unwrap();
                    output.push_str(block.to_str());
                }
                let place = Place::from_usize(zeroes % 4).unwrap();
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
                let place = Place::from_usize(modulo).unwrap();
                output.push_str(place.to_str());

                if num != 1 {
                    output.push_str(num.to_str());
                }
            },
            _ => {
                if idx != 0 {
                    let block = Block::from_usize(idx)
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
