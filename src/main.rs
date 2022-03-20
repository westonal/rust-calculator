extern crate core;

use std::env;
use crate::calculator::Calculator;

mod scratchpad_iter_test;
mod shunting_yard;
mod tokenizer;
mod calculator;

fn main() {
    let string = env::args().skip(1).collect::<Vec<String>>().join(" ");
    let calculator = Calculator {};
    print!("{} = ", string);
    let result = calculator.calculate::<f64>(&string);
    match result {
        Ok(value) => {
            println!("{}", value);
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}
