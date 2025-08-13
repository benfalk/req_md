//!
//! # ReqMD Core
//!
#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

pub mod request {
    pub use ::reqmd_http::header::Headers;
    pub use ::reqmd_http::request::{Body, Method};
    pub use ::url::Host;

    #[derive(Debug, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[non_exhaustive]
    pub struct Request {
        pub server_address: ServerAddress,
        pub method: Method,
        pub path: Path,
        pub query: Query,
        pub headers: Headers,
        pub body: Body,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[non_exhaustive]
    pub struct ServerAddress {
        pub scheme: Scheme,
        pub host: Host,
        pub port: Option<u16>,
    }

    /// # Namespace
    ///
    /// Defines the namespace for the request.  This
    /// is presented as a Prefix in the URL path.
    ///
    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[non_exhaustive]
    pub enum Namespace {
        #[default]
        None,

        Segment(String),
    }

    /// # Scheme
    ///
    /// Web request scheme, either HTTP or HTTPS.
    ///
    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[non_exhaustive]
    pub enum Scheme {
        Http,

        #[default]
        Https,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Default, PartialOrd, Ord)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "serde", serde(transparent))]
    pub struct Path(String);

    impl Path {
        pub fn new(uri: String) -> Self {
            Self(uri)
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    /// # Query Parameters
    /// ```rust
    /// # use reqmd_core::request::Query;
    /// let query = Query::from_iter([
    ///     ("foo", "bar"),
    ///     ("fizz", "buzz"),
    /// ]);
    ///
    /// assert_eq!(query.first_value_for("foo"), Some("bar"));
    /// assert_eq!(query.first_value_for("fizz"), Some("buzz"));
    /// assert!(query.first_value_for("nonexistent").is_none());
    /// ```
    /// ---
    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "serde", serde(transparent))]
    pub struct Query(Vec<QueryParameter>);

    impl Query {
        /// Returns the first occurrence of a value for a given key, if it exists.
        pub fn first_value_for(&self, key: &str) -> Option<&str> {
            self.values_for(key).next()
        }

        /// Provides an iterator over all values for a given key.
        pub fn values_for(&self, key: &str) -> impl Iterator<Item = &str> {
            self.0
                .iter()
                .filter(move |param| param.key.eq(key))
                .map(|param| param.value.as_str())
        }

        /// Adds a new key-value pair to the query.
        pub fn add(&mut self, key: impl Into<String>, value: impl Into<String>) {
            self.0.push(QueryParameter::new(key, value));
        }

        /// Tests if the query is empty.
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }

        /// Return the number of query parameters.
        pub fn len(&self) -> usize {
            self.0.len()
        }

        /// Returns an iterator over the query parameters.
        pub fn iter(&self) -> impl Iterator<Item = &QueryParameter> {
            self.0.iter()
        }

        /// Returns a mutable iterator over the query parameters.
        pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut QueryParameter> {
            self.0.iter_mut()
        }
    }

    impl<T> FromIterator<T> for Query
    where
        T: Into<QueryParameter>,
    {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            Query(iter.into_iter().map(Into::into).collect())
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Default, PartialOrd, Ord)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct QueryParameter {
        pub key: String,
        pub value: String,
    }

    impl QueryParameter {
        pub fn new<K, V>(key: K, value: V) -> Self
        where
            K: Into<String>,
            V: Into<String>,
        {
            QueryParameter {
                key: key.into(),
                value: value.into(),
            }
        }
    }

    impl<T, F> From<(T, F)> for QueryParameter
    where
        T: Into<String>,
        F: Into<String>,
    {
        fn from((key, value): (T, F)) -> Self {
            Self::new(key, value)
        }
    }
}
