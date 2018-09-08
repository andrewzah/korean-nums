#[macro_use]
extern crate clap;
extern crate rand;
use std::fmt;
use clap::App;

mod group;
mod number;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let random = rand::random::<u32>().to_string();
    println!("{}", random);
    println!("{}", get_grouping_or_number(&random));
}

fn get_grouping_or_number(s: &str) -> String {
    let length = s.len();
    if length == 0 { return String::from("") };

    let chars: Vec<char> = s.chars().collect();
    let next = chars[0].clone();

    if all_chars_are_zero(chars) == true {
        return String::from("");
    } else if next.to_string() == "0" {
        return format!("{}", get_grouping_or_number(&s[1..]));
    }

    let copy = s.clone();
    match length {
        1 => {
            let s1 = format!("{}", number::Number::from_str(&copy).unwrap().to_str_sino().unwrap());
            format!("{}{}", s1, get_grouping_or_number(&copy[1..]))
        }
        _ => {
            let s1 = format!("{}{}", &copy.chars().next().unwrap(), group::Group::from_length(length).unwrap().to_str_sino());
            format!("{}{}", s1, get_grouping_or_number(&copy[1..]))
        }
    }
}

fn all_chars_are_zero(vec: Vec<char>) -> bool {
    vec.into_iter().all(|v| v.to_string() == "0")
}

