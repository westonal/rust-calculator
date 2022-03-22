extern crate core;

use std::env;
use rustyline::Editor;
use rustyline::error::ReadlineError;
use crate::calculator::Calculator;

mod shunting_yard;
mod tokenizer;
mod calculator;

const BACKSPACE: char = 8u8 as char;

fn main() {
    if env::args().count() == 1 {
        terminal_mode();
    } else {
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
}

fn terminal_mode() {
    // `()` can be used when no completer is required
    let mut editor = Editor::<()>::new();
    if editor.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let prompt = "> ";
        // TODO COLORS?
        // editor.set_color_mode(ColorMode::Enabled);
        let line = editor.readline(prompt);
        match line {
            Ok(line) => {
                if line.is_empty() {
                    println!("{}\r{}", BACKSPACE, BACKSPACE);
                    break;
                }
                if line == "help" {
                    editor.clear_history();
                    println!("  Calculator - Alan Evans 2022");
                    println!("  Terminal mode");
                    println!("    Enter mathematical expression and press enter");
                    println!("    clear - Clear expression history");
                    println!("    help  - this message");
                    println!("    enter - Exit terminal mode");
                    continue;
                }
                if line == "clear" {
                    editor.clear_history();
                    println!("History cleared");
                    continue;
                }
                editor.add_history_entry(line.as_str());
                let calculator = Calculator {};
                let result = calculator.calculate::<f64>(&line);
                match result {
                    Ok(value) => {
                        println!("{}\r{}{} = {}", BACKSPACE, prompt, line, value);
                    }
                    Err(error) => {
                        println!("Error: {}", error);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    editor.save_history("history.txt").unwrap();
}
