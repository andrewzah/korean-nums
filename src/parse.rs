use crate::numbers;
use crate::block;
use crate::place;

use std::cmp::min;
use crate::math::{Sign};

pub fn parse_hangeul_sino(numbers: Vec<char>) -> String {
    println!("new num");
    let len = numbers.len() - 1;
    let mut output = String::new();
    let mut iter = numbers.iter().enumerate().peekable();

    while let Some((idx, input_num)) = iter.next() {
        if let Some(sign) = Sign::from_char(&input_num) {
            if idx != len { panic!("+ or - symbol isn't at the beginning of the input."); }
            output.push_str(&sign.to_string_rev());
            continue;
        }

        let mut mut_idx = idx;
        let remaining = len - mut_idx;
        let min_peek_len = min(remaining, 3);
        let mut num = numbers::KoreanNumberSino::from_char(input_num).unwrap();

        if min_peek_len != 0 && num == 0 {
            let mut zeroes = 1;
            while let Some((next_idx, next_num)) = iter.next() {
                if *next_num != '0' {
                    num = numbers::KoreanNumberSino::from_char(next_num).unwrap();
                    mut_idx = next_idx;
                    break;
                }
                zeroes += 1;
            }
            zeroes += idx;

            if let Some(block) = block::Block::from_usize(zeroes) {
                // special edge case for 만
                if block.to_str() == "만" {
                    if output != "" {
                        if mut_idx == len - 1 && output.chars().collect::<Vec<char>>()[0] != ' '  {
                            if let Some((_, peek_num)) = iter.peek() {
                                match *peek_num {
                                    '0'..='9' => { output.push(' '); },
                                    _ => {}
                                }
                            }
                        }
                    }

                    if num != 1 && mut_idx != len - 1 && output != "" {
                        output.push(' ');
                    } 
                }
                output.push_str(block.to_str());

                if let Some((_, peek_num)) = iter.peek() {
                    match *peek_num {
                        '0'..='9' => { output.push_str(num.to_str()); },
                        _ => {}
                    }
                } else {
                    if num != 1 || block.to_str() != "만" {
                        output.push_str(num.to_str());
                    }
                }

                continue;
            } else {
                let zmod = zeroes % 4;
                if zeroes >= 4 {
                    let block = block::Block::from_usize(zeroes - zmod).unwrap();
                    if output.len() > 2 {
                        let chars = output.chars().rev().collect::<String>();
                        if chars[3..6] != *"만" {
                            output.push(' ');
                            output.push_str(block.to_str());
                        }
                    } else {
                        output.push_str(block.to_str());
                    }
                }

                let place = place::Place::from_usize(zeroes % 4).unwrap();
                output.push_str(place.to_str());

                if num != 1 {
                    output.push_str(num.to_str());
                }

                continue;
            }
        }

        let modulo = mut_idx % 4;
        match modulo {
            1|2|3 => {
                let place = place::Place::from_usize(modulo).unwrap();
                output.push_str(place.to_str());

                if num != 1 {
                    output.push_str(num.to_str());
                }
            },
            _ => {
                if mut_idx != 0 {
                    println!("bottom reached");
                    let block = block::Block::from_usize(mut_idx)
                        .expect("Block counter doesn't go high enough for this...");
                    output.push_str(&block.to_str_with_space());
                    if num != 1 || block.to_str() != "만" || remaining > 0 {
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
