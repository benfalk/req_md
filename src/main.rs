mod application;
mod parser;
mod req;
mod variables;
mod pretty_output;

use application::OutputFormat::{Raw, MarkDown};
use pretty_output::PrettyOutput;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let opts = application::get_opts();

    if opts.list_requests {
        list_requests(&opts);
    } else {
        run_request(&opts);
    }
}

fn list_requests(opts: &application::Opts) {
    let data = opts.input().unwrap();
    let vars = variables::Variables::new(&data);
    let mut reqs = parser::parse_requests(&vars.expand(&data));

    match opts.at_line() {
        None => {
            for req in &mut reqs {
                opts.apply_overrieds(req);
                println!("{:#?}", req);
            }
        },
        Some(line_number) => {
            for req in &mut reqs {
                if req.meta.line_range.contains(&line_number) {
                    opts.apply_overrieds(req);
                    println!("{:#?}", req);
                    return;
                }
            }
        }
    }
}

fn run_request(opts: &application::Opts) {
    let data = opts.input().unwrap();
    let vars = variables::Variables::new(&data);
    let mut reqs = parser::parse_requests(&vars.expand(&data));
    let line = opts.at_line().unwrap_or(1);


    for req in &mut reqs {
        if req.meta.line_range.contains(&line) {
            opts.apply_overrieds(req);
            match req.send() {
                Ok(resp) => {
                    match opts.output {
                        Raw => println!("{}", resp.text().unwrap()),
                        MarkDown => println!("{}", PrettyOutput::pretty_output(resp)),
                    }
                },
                Err(err) => eprintln!("{}", err),
            }
            return;
        }
    }

    if let Some(req) = reqs.iter().nth(0) {
        match req.send() {
            Ok(resp) => {
                match opts.output {
                    Raw => println!("{}", resp.text().unwrap()),
                    MarkDown => println!("{}", PrettyOutput::pretty_output(resp)),
                }
            },
            Err(err) => eprintln!("{}", err),
        }
    }
}
