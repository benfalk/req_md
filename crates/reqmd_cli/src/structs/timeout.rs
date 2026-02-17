use ::std::{str::FromStr, time::Duration};

/// # Timeout Duration
///
/// Signifies a duration for timeouts which clap
/// can parse from user input.  Can parse values
/// such as `15sec`, `300ms`, `2min`, or `none`.
///
/// ```rust
/// # use reqmd_cli::TimeoutDuration;
/// # use std::time::Duration;
/// let empty = "".parse::<TimeoutDuration>().unwrap();
/// assert_eq!(empty.duration, None);
///
/// let infinite = "none".parse::<TimeoutDuration>().unwrap();
/// assert_eq!(infinite.duration, None);
///
/// let fifty_ms = "50ms".parse::<TimeoutDuration>().unwrap();
/// assert_eq!(fifty_ms.duration, Some(Duration::from_millis(50)));
///
/// let two_sec = "2sec".parse::<TimeoutDuration>().unwrap();
/// assert_eq!(two_sec.duration, Some(Duration::from_secs(2)));
///
/// let three_min = "3min".parse::<TimeoutDuration>().unwrap();
/// assert_eq!(three_min.duration, Some(Duration::from_secs(3 * 60)));
/// ```
/// ---
#[derive(Debug, Clone, Default)]
pub struct TimeoutDuration {
    pub duration: Option<Duration>,
}

impl FromStr for TimeoutDuration {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() || s.trim() == "none" {
            return Ok(TimeoutDuration { duration: None });
        }

        let amount = s
            .trim_end_matches(char::is_alphabetic)
            .parse::<u64>()
            .map_err(|_| "invalid number for timeout duration")?;

        if s.ends_with("ms") {
            Ok(TimeoutDuration {
                duration: Some(Duration::from_millis(amount)),
            })
        } else if s.ends_with("sec") {
            Ok(TimeoutDuration {
                duration: Some(Duration::from_secs(amount)),
            })
        } else if s.ends_with("min") {
            Ok(TimeoutDuration {
                duration: Some(Duration::from_secs(amount * 60)),
            })
        } else {
            Err("invalid timeout duration suffix, use ms, sec or min")
        }
    }
}
