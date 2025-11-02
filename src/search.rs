use crate::{api, player};
use std::io::{Write, stdin, stdout};

pub fn search_menu() {
    print!("Enter search query: ");
    stdout().flush().unwrap();

    let mut query = String::new();
    stdin().read_line(&mut query).unwrap();
    let query = query.trim(); 

    let results = api::search(query, 1);

    if results.is_empty() {
        println!("No results found.");
        return;
    }

    for (i, (title, url)) in results.iter().enumerate() {
        println!("{}: {} -> {}", i + 1, title, url);
    }

    print!("\nPick a number to play: ");
    stdout().flush().unwrap();

    let mut pick = String::new();
    stdin().read_line(&mut pick).unwrap();

    if let Ok(i) = pick.trim().parse::<usize>() {
        if i > 0 && i <= results.len() {
            let (_, url) = &results[i - 1];
            player::play(url);
        }
    }
}
