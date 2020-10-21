#![feature(str_split_once)]
use std::io::{self, Read};

mod parser;
mod req;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    if let Some(req) = parser::parse_request(&buffer) {
        if let Ok(resp) = req.send() {
            println!("{}", resp.text().unwrap());
        }
    } else {
        eprintln!("Unable to parse a request!");
    };
}
