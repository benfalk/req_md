/// # Query Parameters
/// ```rust
/// # use reqmd_http::request::QueryString;
/// let query = QueryString::from_iter([
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
pub struct QueryString(Vec<QueryParameter>);

impl QueryString {
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

impl<T> FromIterator<T> for QueryString
where
    T: Into<QueryParameter>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        QueryString(iter.into_iter().map(Into::into).collect())
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
