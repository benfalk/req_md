use clap::Clap;
use std::fs::File;
use std::io::{self, Read};

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Ben Falk <ben.falk@yahoo.com>")]
pub struct Opts {
    /// file to use for a request
    file: Option<String>,
}

pub fn get_opts() -> Opts {
    Opts::parse()
}

impl Opts {
    pub fn input(&self) -> Option<String> {
        let mut data = String::new();
        if let Some(filename) = &self.file {
            let mut file = File::open(filename).unwrap();
            file.read_to_string(&mut data).unwrap();
            Some(data)
        } else if !atty::is(atty::Stream::Stdin) {
            io::stdin().read_to_string(&mut data).unwrap();
            Some(data)
        } else {
            None
        }
    }
}
