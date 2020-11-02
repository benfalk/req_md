#![feature(str_split_once)]
mod application;
mod parser;
mod req;

fn main() {
    let opts = application::get_opts();

    if opts.list_requests {
        list_requests(&opts);
    } else {
        run_request(&opts);
    }
}

fn list_requests(opts: &application::Opts) {
    let data = opts.input().unwrap();
    let reqs = parser::parse_requests(&data);

    for req in reqs {
        println!("{:#?}", req);
    }
}

fn run_request(opts: &application::Opts) {
    let data = opts.input().unwrap();
    let reqs = parser::parse_requests(&data);
    let line = opts.at_line().unwrap_or(1);


    for req in &reqs {
        if req.meta.line_range.contains(&line) {
            match req.send() {
                Ok(resp) => println!("{}", resp.text().unwrap()),
                Err(err) => eprintln!("{}", err),
            }
            return;
        }
    }

    if let Some(req) = reqs.iter().nth(0) {
        match req.send() {
            Ok(resp) => println!("{}", resp.text().unwrap()),
            Err(err) => eprintln!("{}", err),
        }
    }
}
