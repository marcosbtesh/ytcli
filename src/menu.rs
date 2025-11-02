use crate::{search, player};
use std::io::{stdin, stdout, Write};


pub fn run() {
    loop{
        println!("=== YoutubeCLI ===");
        println!("Search");
        println!("URL");
        println!("Quit");
        println!("Choice: ");
        stdout.flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "Search" => serach::search_menu(),
            "URL" =>  {
                print!("Youtube URL: ");
                stdout.flush().unwrap();
                let mut url = String::new();
                stdin.read_line(&mut url).unwrap();
                Player::play(&url.trim());
            },
            "Quit": break,
            _ => println!("Invalid");
        }
    }
}
