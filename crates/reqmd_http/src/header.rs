/// A list of key-value pairs representing HTTP headers.
/// ```rust
/// # use reqmd_http::Headers;
///
/// // Prepares a new Headers instance with a specified capacity
/// let mut headers = Headers::with_capacity(2);
/// headers.add("Content-Type", "text/plain");
/// headers.add("Authorization", "Bearer token");
///
/// assert_eq!(headers.len(), 2);
/// assert_eq!(headers.first("Content-Type"), Some("text/plain"));
///
/// // Supports iterating over headers for updates
/// for header in headers.iter_mut() {
///     if header.key.eq_ignore_ascii_case("Authorization") {
///         header.value = String::from("SECRET");
///     }
/// }
///
/// // Key lookup is case-insensitive
/// assert_eq!(headers.first("authorization"), Some("SECRET"));
///
/// // Accessing headers by index
/// assert_eq!(headers[0].key, "Content-Type");
/// assert_eq!(headers[0].value, "text/plain");
///
/// // Creates a Headers instance from an iterator of tuples
/// let headers = Headers::from_iter([
///     ("X-Custom-Header", "value1"),
///     ("X-Another-Header", "value2")
/// ]);
///
/// assert_eq!(headers.first("x-custom-header"), Some("value1"));
/// assert_eq!(headers.first("x-another-header"), Some("value2"));
/// ```
/// ---
#[derive(Debug, Clone, PartialEq, Default, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Headers(Vec<HeaderLine>);

impl Headers {
    /// Prepares a new `Headers` instance with a specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Headers(Vec::with_capacity(capacity))
    }

    /// Provides an iterator over all header values for a given key.
    pub fn values_for(&self, key: &str) -> impl Iterator<Item = &str> {
        self.0
            .iter()
            .filter(|header| header.key.eq_ignore_ascii_case(key))
            .map(|header| header.value.as_str())
    }

    /// Provides the first value for a given key, if it exists.
    pub fn first(&self, key: &str) -> Option<&str> {
        self.values_for(key).next()
    }

    /// Adds a new header line with the specified key and value.
    pub fn add(&mut self, key: &str, value: &str) {
        self.0.push(HeaderLine::new(key, value));
    }

    /// Adds a new header line from a tuple of key and value.
    pub fn insert_many<I, T>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
        T: Into<HeaderLine>,
    {
        self.0.extend(iter.into_iter().map(Into::into));
    }

    /// Tests if the headers collection is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the number of headers in the collection.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Reference iterator for the headers collection.
    pub fn iter(&self) -> impl Iterator<Item = &HeaderLine> {
        self.0.iter()
    }

    /// Mutable reference iterator for the headers collection.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut HeaderLine> {
        self.0.iter_mut()
    }
}

/// Represents a single header line with a key and value.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HeaderLine {
    pub key: String,
    pub value: String,
}

impl HeaderLine {
    #[doc(hidden)]
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        HeaderLine {
            key: key.into(),
            value: value.into(),
        }
    }
}

impl<K, V> From<(K, V)> for HeaderLine
where
    K: Into<String>,
    V: Into<String>,
{
    fn from((key, value): (K, V)) -> Self {
        HeaderLine::new(key, value)
    }
}

impl std::ops::Index<&str> for Headers {
    type Output = HeaderLine;

    fn index(&self, key: &str) -> &Self::Output {
        self.0
            .iter()
            .find(|header| header.key.eq_ignore_ascii_case(key))
            .expect("Header not found")
    }
}

impl std::ops::Index<usize> for Headers {
    type Output = HeaderLine;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IntoIterator for Headers {
    type Item = HeaderLine;
    type IntoIter = std::vec::IntoIter<HeaderLine>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Headers {
    type Item = &'a HeaderLine;
    type IntoIter = std::slice::Iter<'a, HeaderLine>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<T> FromIterator<T> for Headers
where
    T: Into<HeaderLine>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Headers(iter.into_iter().map(Into::into).collect())
    }
}
