#![feature(str_split_once)]
mod application;
mod parser;
mod req;

fn main() {
    let opts = application::get_opts();
    let data = opts.input().unwrap();

    if let Some(req) = parser::parse_request(&data) {
        if let Ok(resp) = req.send() {
            println!("{}", resp.text().unwrap());
        }
    } else {
        eprintln!("Unable to parse a request!");
    };
}
