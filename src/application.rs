use clap::Parser;
use std::fs::File;
use std::io::{self, Read};
use std::time::Duration;
use crate::req::Request;

#[derive(Clone, Copy)]
pub enum OutputFormat {
    Raw,
    MarkDown
}

#[derive(Debug, Clone)]
pub struct TimeoutDuration {
    pub duration: Duration,
}

#[derive(Parser)]
#[command(version, author = "Ben Falk <benjamin.falk@yahoo.com>")]
pub struct Opts {
    /// file to use for a request
    file: Option<String>,

    /// list all requests parsed from input w/o running them
    #[clap(long)]
    pub list_requests: bool,

    /// At what line number do you want to run a request
    #[clap(long)]
    pub line: Option<u32>,

    /// options are 'raw' and 'markdown'
    #[clap(long, default_value = "raw")]
    pub output: OutputFormat,

    /// optional, examples 15sec 300ms 2min
    #[clap(long)]
    pub timeout: Option<TimeoutDuration>,
}

pub fn get_opts() -> Opts {
    Opts::parse()
}

impl Opts {
    pub fn input(&self) -> Option<String> {
        let mut data = String::new();
        if let Some(filename) = &self.file {
            let filename = filename.split(":").nth(0)?;
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

    pub fn apply_overrieds(&self, request: &mut Request) {
        if self.timeout.is_some() {
            request.meta.timeout = self.timeout.clone();
        }
    }

    pub fn at_line(&self) -> Option<u32> {
        if self.line.is_some() {
            return self.line;
        }

        self.file
            .as_ref()
            .take()?
            .split(":")
            .nth(1)?
            .parse()
            .map_or(None, |n| Some(n))
    }
}

use std::str::FromStr;

impl FromStr for OutputFormat {
    type Err = &'static str;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.to_lowercase().as_str() {
            "raw" => Ok(OutputFormat::Raw),
            "markdown" => Ok(OutputFormat::MarkDown),
            _ => Err("not a valid output format"),
        }
    }
}

impl FromStr for TimeoutDuration {
    type Err = &'static str;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let amount:u64 = string
            .trim_end_matches(char::is_alphabetic)
            .parse()
            .map_err(|_| "no valid number found")?;

        if string.ends_with("sec") {
            Ok(Self { duration: Duration::from_secs(amount) })
        }
        else if string.ends_with("ms") {
            Ok(Self { duration: Duration::from_millis(amount) } )
        }
        else if string.ends_with("min") {
            Ok(Self { duration: Duration::from_secs(amount * 60) } )
        }
        else {
            Err("Not a valid duration")
        }
    }
}
