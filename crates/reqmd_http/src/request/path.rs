use super::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path(Url);

impl Path {
    pub fn as_str(&self) -> &str {
        self.0.path()
    }

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
