use crate::{search, player};
use std::io::{stdin, stdout, Write};

pub fn run() {
    loop {
        println!("==== YouTube CLI ====");
        println!("1) Search");
        println!("2) Play by URL");
        println!("3) Quit");
        print!("Choice: ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => search::search_menu(),
            "2" => {
                print!("YouTube URL: ");
                stdout().flush().unwrap();
                let mut url = String::new();
                stdin().read_line(&mut url).unwrap();
                player::play(&url.trim());
            }
            "3" => break,
            _ => println!("Invalid"),
        }
    }
}