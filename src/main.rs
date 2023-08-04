mod string_utils;

use std::io::{stdin, stdout, Write};

use string_utils::OobeString;

fn main() {
    loop {
        let mail_response = prompt("Do you use Gmail or Hotmail?");
        match mail_response.to_lowercase().as_str() {
            "gmail" | "g" => {
                println!("Setting up Gmail...");
                break;
            }
            "hotmail" | "h" => {
                println!("Setting up Hotmail...");
                break;
            }
            _ => {}
        }
    }

    let office_response = prompt("Do you need an office suite?");
    if office_response.is_yes() {
        let documents = prompt("Documents?");
        let slideshows = prompt("Slideshows?");
        let spreadsheets = prompt("Spreadsheets?");

        if documents.is_yes() {
            println!("Setting up Libre Office Writer...")
        }

        if slideshows.is_yes() {
            println!("Setting up Libre Office Impress...");
        }

        if spreadsheets.is_yes() {
            println!("Setting up Libre Office Calc...");
        }
    }

    let zoom_response = prompt("Are you going to be making video calls?");
    if zoom_response.is_yes() {
        println!("Setting up Zoom...");
    }
}

// Prompts the user for input and returns their response
fn prompt(message: &str) -> String {
    print!("{} ", message);
    flush();

    read_line()
}

// Reads a line of input from the user (just a more ergonomic usage of `Stdin::read_line()`)
fn read_line() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    input.trim().to_owned()
}

// Flushes stdout, printing all text in the buffer
fn flush() {
    stdout().flush().unwrap();
}
