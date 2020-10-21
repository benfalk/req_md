#![feature(str_split_once)]
#![allow(dead_code)]
use std::io::{self, Read};

mod req;
mod parser;
mod new_parser;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    // new_parser::parse_request(&buffer);

    if let Some(req) = parser::parse_request(&buffer) {
        if let Ok(resp) = req.send() {
            println!("{}", resp.text().unwrap());
        }
    }
    else {
        eprintln!("Unable to parse a request!");
    };
}
