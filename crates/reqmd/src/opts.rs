use ::std::{str::FromStr, time::Duration};

#[derive(::clap::Parser, Debug)]
pub struct Opts {
    /// file to read reaquests from,
    /// if not provided stdin is used
    file: Option<String>,

    /// list all requests parsed from
    /// input without running them
    #[clap(long)]
    pub list_requests: bool,
}

/// # Timeout Duration
///
/// Signifies a duration for timeouts which clap
/// can parse from user input.  Can parse values
/// such as `15sec`, `300ms`, `2min`.
///
/// ---
#[derive(Debug, Clone)]
pub struct TimeoutDuration {
    pub duration: Duration,
}

impl FromStr for TimeoutDuration {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let amount = s
            .trim_end_matches(char::is_alphabetic)
            .parse::<u64>()
            .map_err(|_| "invalid number for timeout duration")?;

        if s.ends_with("ms") {
            Ok(TimeoutDuration {
                duration: Duration::from_millis(amount),
            })
        } else if s.ends_with("sec") {
            Ok(TimeoutDuration {
                duration: Duration::from_secs(amount),
            })
        } else if s.ends_with("min") {
            Ok(TimeoutDuration {
                duration: Duration::from_secs(amount * 60),
            })
        } else {
            Err("invalid timeout duration suffix, use ms, sec or min")
        }
    }
}
