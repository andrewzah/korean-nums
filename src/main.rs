#[macro_use]
extern crate clap;
extern crate korean_nums;
extern crate rand;

use clap::App;

use korean_nums::{calculate, str_to_vec_string, u32_to_vec_string};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(numbers) = matches.values_of("numbers") {
        for num in numbers {
            println!("{}", calculate(str_to_vec_string(num)));
        }
    } else {
        let strings = u32_to_vec_string(rand::random::<u32>());

        println!("{}", strings.join(""));
        println!("{}", calculate(strings));
    }
}
