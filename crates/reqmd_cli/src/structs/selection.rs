use ::reqmd_core::{MdRequest, MdRequestList};
use ::std::{num::NonZeroUsize, path::PathBuf, str::FromStr};

/// Describes a [selection] in a markdown file should be
/// considered for an operation.  This can be parsed from
/// a string with the following format:
///
/// `"{filepath}:{selection}"`
///
/// ## Examples
///
/// ```rust
/// # use ::reqmd_cli::{Target, Selection};
/// # use ::std::num::NonZeroUsize;
/// let first = "sample.md:first".parse::<Target>().unwrap();
/// assert_eq!(first.file.as_os_str(), "sample.md");
/// assert!(matches!(first.selection, Selection::First));
///
/// let last = "sample.md:last".parse::<Target>().unwrap();
/// assert_eq!(last.file.as_os_str(), "sample.md");
/// assert!(matches!(last.selection, Selection::Last));
///
/// let fourth = "sample.md:4".parse::<Target>().unwrap();
/// let four = NonZeroUsize::try_from(4).unwrap();
/// assert_eq!(fourth.file.as_os_str(), "sample.md");
/// assert!(matches!(fourth.selection, Selection::Nth(four)));
///
/// let line_42 = "sample.md:line42".parse::<Target>().unwrap();
/// let forty_two = NonZeroUsize::try_from(42).unwrap();
/// assert_eq!(line_42.file.as_os_str(), "sample.md");
/// assert!(matches!(line_42.selection, Selection::Line(forty_two)));
/// ```
///
/// [selection]:  Selection
/// ---
#[derive(Debug, Clone)]
pub struct Target {
    pub file: PathBuf,
    pub selection: Selection,
}

/// Describes a selection of a [request] in a markdown file.
/// This can be a line number, an ordinal number, or the first
/// or last request in the file.
///
/// ---
#[derive(Debug, Clone, Copy)]
pub enum Selection {
    Line(NonZeroUsize),
    Nth(NonZeroUsize),
    First,
    Last,
}

impl Selection {
    /// attempts to select a request from the given list of requests
    pub fn select(self, list: &MdRequestList) -> Option<&MdRequest> {
        match self {
            Self::First => list.iter().next(),
            Self::Last => list.iter().next_back(),
            Self::Line(line) => list.at_line(usize::from(line)),
            Self::Nth(n) => list.iter().nth(usize::from(n) - 1),
        }
    }
}

impl FromStr for Target {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (path_part, selection_part) =
            s.split_once(":").ok_or("target requires `:` delimiter")?;

        Ok(Self {
            file: path_part.into(),
            selection: selection_part.parse()?,
        })
    }
}

impl FromStr for Selection {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "first" => Self::First,
            "last" => Self::Last,
            line if line.starts_with("line") => {
                let num = line[4..]
                    .parse()
                    .map_err(|_| "line numer must be a non-zero number")?;
                Self::Line(num)
            }
            nth => {
                let num = nth
                    .parse()
                    .map_err(|_| "selection must be a non-zero number")?;
                Self::Nth(num)
            }
        })
    }
}
