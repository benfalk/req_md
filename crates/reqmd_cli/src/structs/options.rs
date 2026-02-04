use crate::structs::TimeoutDuration;
use ::anyhow::{Context as _, Result, bail};
use ::std::{
    io::{IsTerminal, Read},
    path::PathBuf,
};
use std::time::Duration;

#[derive(::clap::Parser, Debug)]
#[command(version, author = "Ben Falk <benjamin.falk@yahoo.com>")]
pub struct Options {
    /// file to read reaquests from,
    /// if not provided stdin is used
    file: Option<String>,

    /// list all requests parsed from
    /// input without running them
    #[clap(long)]
    list_requests: bool,

    /// optional timeout duration
    /// examples are 10sec, 500ms, 2min
    #[clap(long)]
    timeout: Option<TimeoutDuration>,
}

impl Options {
    pub fn markdown(&self) -> Result<String> {
        if self.file.is_none() && ::std::io::stdin().is_terminal() {
            bail!("no input file provided and stdin is a terminal");
        }

        let data = match self.file_path() {
            Some(path) => ::std::fs::read_to_string(&path)
                .with_context(|| format!("failed to read file at path: {path:?}"))?,
            None => {
                let mut buffer = String::new();
                ::std::io::stdin()
                    .read_to_string(&mut buffer)
                    .context("failed to read from stdin")?;
                buffer
            }
        };

        Ok(data)
    }

    pub fn list_requests(&self) -> bool {
        self.list_requests
    }

    pub fn timeout_duration(&self) -> Option<Duration> {
        self.timeout.as_ref().map(|t| t.duration)
    }

    fn file_path(&self) -> Option<PathBuf> {
        self.file
            .as_ref()
            .and_then(|file| file.split(":").nth(0).map(Into::into))
    }

    pub fn file_line(&self) -> Result<Option<usize>> {
        let Some((_, line_str)) =
            self.file.as_ref().and_then(|file| file.split_once(":"))
        else {
            return Ok(None);
        };

        let line_num = line_str.parse::<usize>().with_context(|| {
            format!("failed to parse line number from file specifier: {line_str}")
        })?;

        Ok(Some(line_num))
    }
}
