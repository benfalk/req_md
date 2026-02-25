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

    /// Provides a mutable iterator over all header values for a given key.
    ///
    /// ```rust
    /// # use reqmd_http::Headers;
    /// let mut headers = Headers::from_iter([
    ///     ("foo", "bar"),
    ///     ("biz", "buz"),
    ///     ("foo", "rab")
    /// ]);
    ///
    /// for var in headers.values_for_mut("foo") {
    ///     if var != "bar" {
    ///         var.make_ascii_uppercase();
    ///     }
    /// }
    ///
    /// assert_eq!(headers.first("foo"), Some("bar"));
    /// assert_eq!(headers.first("biz"), Some("buz"));
    ///
    /// let foos = headers.values_for("foo").collect::<Vec<_>>();
    /// assert_eq!(foos, vec!["bar", "RAB"]);
    /// ```
    /// ---
    pub fn values_for_mut(
        &mut self,
        key: &str,
    ) -> impl Iterator<Item = &mut String> {
        self.0
            .iter_mut()
            .filter(|header| header.key.eq_ignore_ascii_case(key))
            .map(|header| &mut header.value)
    }

    /// Returns a mutable reference to the first value for a given
    /// key, if it exists.
    ///
    /// Provides a convenient way to update the value the
    /// first header line matching the specified key.
    ///
    /// ```rust
    /// # use reqmd_http::Headers;
    /// let mut headers = Headers::from_iter([
    ///     ("foo", "bar"),
    ///     ("biz", "buz"),
    ///     ("foo", "rab")
    /// ]);
    ///
    /// let Some(foo) = headers.first_mut("foo") else {
    ///     panic!("Expected to find a header with key 'foo'");
    /// };
    /// foo.make_ascii_uppercase();
    /// let foos = headers.values_for("foo").collect::<Vec<_>>();
    /// assert_eq!(foos, vec!["BAR", "rab"]);
    /// ```
    /// ___
    pub fn first_mut(&mut self, key: &str) -> Option<&mut String> {
        self.values_for_mut(key).next()
    }

    /// Removes the first header line matching a key and returns
    /// it's value if found.
    ///
    /// ```rust
    /// # use reqmd_http::Headers;
    /// let mut headers = Headers::from_iter([
    ///     ("foo", "bar"),
    ///     ("biz", "buz"),
    ///     ("foo", "rab")
    /// ]);
    ///
    /// let maybe_foo = headers.delete_first("foo");
    /// assert_eq!(maybe_foo.as_deref(), Some("bar"));
    /// assert_eq!(headers.first("biz"), Some("buz"));
    /// assert_eq!(headers.first("foo"), Some("rab"));
    ///
    /// let maybe_foo = headers.delete_first("foo");
    /// assert_eq!(maybe_foo.as_deref(), Some("rab"));
    /// assert_eq!(headers.first("biz"), Some("buz"));
    /// assert!(headers.first("foo").is_none());
    /// ```
    /// ---
    pub fn delete_first(&mut self, key: &str) -> Option<String> {
        self.0
            .iter()
            .position(|header| header.key.eq_ignore_ascii_case(key))
            .map(|pos| self.0.remove(pos).value)
    }

    /// Removes all header lines matching a key and returns their values.
    ///
    /// ```rust
    /// # use reqmd_http::Headers;
    ///
    /// let mut headers = Headers::from_iter([
    ///     ("foo", "bar"),
    ///     ("biz", "buz"),
    ///     ("foo", "rab")
    /// ]);
    ///
    /// let foos = headers.delete_all("foo");
    ///
    /// assert_eq!(foos, vec!["bar".to_string(), "rab".to_string()]);
    /// assert_eq!(headers.first("biz"), Some("buz"));
    /// assert!(headers.first("foo").is_none());
    /// ```
    /// ---
    pub fn delete_all(&mut self, key: &str) -> Vec<String> {
        let mut deleted_values = Vec::with_capacity(4);
        self.0.retain_mut(|header| {
            if header.key.eq_ignore_ascii_case(key) {
                let mut deleted = String::new();
                std::mem::swap(&mut deleted, &mut header.value);
                deleted_values.push(deleted);
                false
            } else {
                true
            }
        });
        deleted_values
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
