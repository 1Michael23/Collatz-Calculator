// used to compute the collatz conjecture

use std::io;
fn main() {
    
    println!("Please Input A Number:");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input.");



    let mut input: u128 = input.trim().parse().expect("invalid input");


    while input !=1 {

       if input % 2 == 0 {
            input = input / 2;
        } else {
            input = input * 3 + 1;
        }

        println!("{}", input)

    }
    
}