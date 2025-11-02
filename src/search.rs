use crate::{api, player};
use std::io::{Write, stdin, stdout};

pub fn search_menu() {
    println!("Enter search query: ");
    stdout().flush().unwrap();

    let mut query = String::new();
    stdin().read_line(&mut query).unwrap();
    let results = api::search(query.trim());
}
