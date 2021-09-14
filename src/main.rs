// used to compute the collatz conjecture

use clap::{App, Arg};

fn main() {
    
    let number = App::new("Collatz-Calculator").arg(Arg::with_name("input").index(1).takes_value(true).required(true)).get_matches();

    let mut input: u128 = number.value_of("input").unwrap().trim().parse().expect("invalid input");

    while input !=1 {

       if input % 2 == 0 {
            input = input / 2;
        } else {
            input = input * 3 + 1;
        }

        println!("{}", input)

    }
    
}