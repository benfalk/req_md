use super::Url;

/// # Http Path
///
/// Represents the part of a URL that specifies a
/// resource or endpoint on a server. It comes after
/// the domain name and port (if specified) in a URL
/// and is used to identify a specific resource.
///
/// ```rust
/// # use reqmd_http::Path;
/// let mut path = Path::default();
/// assert_eq!(path.as_str(), "/");
///
/// path.append("/api/v1/resource");
/// assert_eq!(path.as_str(), "/api/v1/resource");
/// ```
///
/// `NOTE:` Does not include data [super::QueryString]
///
/// ---
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path(Url);

impl Path {
    /// string reference of path
    pub fn as_str(&self) -> &str {
        self.0.path()
    }

    /// attaches segment(s) string to existing path
    /// ```rust
    /// # use reqmd_http::Path;
    /// let mut path = Path::from("/api/v1");
    /// path.append("resource");
    /// assert_eq!(path.as_str(), "/api/v1/resource");
    /// ```
    /// ---
    pub fn append(&mut self, segment: &str) {
        let mut path = self.0.path().to_string();
        if !path.ends_with('/') {
            path.push('/');
        }

        match segment.starts_with('/') {
            true => {
                path.push_str(&segment[1..]);
            }
            false => {
                path.push_str(segment);
            }
        }

        self.0.set_path(&path);
    }

    /// determines if path is the default "/"
    /// ```rust
    /// # use reqmd_http::Path;
    /// assert!(Path::default().is_root());
    /// assert!(Path::from("").is_root());
    /// assert!(!Path::from("/api").is_root());
    /// ```
    /// ---
    pub fn is_root(&self) -> bool {
        self.as_str() == "/"
    }
}

mod trait_impls {
    use super::*;

    impl Default for Path {
        fn default() -> Self {
            Path(Url::parse("http://localhost/").expect("Failed to parse default PATH"))
        }
    }

    impl AsRef<str> for Path {
        fn as_ref(&self) -> &str {
            self.0.path()
        }
    }

    impl From<Url> for Path {
        fn from(url: Url) -> Self {
            Path(url)
        }
    }

    impl From<String> for Path {
        fn from(value: String) -> Self {
            let mut path = Self::default();
            path.0.set_path(&value);
            path
        }
    }

    impl From<&str> for Path {
        fn from(value: &str) -> Self {
            let mut path = Self::default();
            path.0.set_path(value);
            path
        }
    }

    impl std::fmt::Display for Path {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0.path())
        }
    }
}

#[cfg(feature = "serde")]
mod serde_impls {
    use super::*;
    use ::serde::{Deserialize, Serialize};
    use std::borrow::Cow;

    type BorrowedPath<'a> = Cow<'a, str>;

    impl Serialize for Path {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }

    impl<'de> Deserialize<'de> for Path {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let s = BorrowedPath::deserialize(deserializer)?;
            Ok(Path::from(s.as_ref()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_path() {
        let path = Path::default();
        assert_eq!(path.as_str(), "/");
    }

    #[test]
    fn from_a_str() {
        let path = Path::from("/api/v1/resource");
        assert_eq!(path.as_str(), "/api/v1/resource");
    }

    #[test]
    fn from_a_relative_path() {
        let path = Path::from("../../api/v1/resource");
        assert_eq!(path.as_str(), "/api/v1/resource");
    }
}
