#[macro_use]
extern crate clap;
extern crate rand;
extern crate korean_nums;

use clap::App;

use korean_nums::str_to_hangul;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(inputs) = matches.values_of("numbers") {
        for input in inputs {
            println!("Handling {}.", input);
            println!("->\t{}", str_to_hangul(input));
        }
    }
    else {
        let string = rand::random::<u32>()
            .to_string();

        println!("{}", str_to_hangul(&string));
    }
}

