/// # Query Parameters
///
/// ```rust
/// # use reqmd_http::QueryString;
/// let query = QueryString::from_iter([
///     ("foo", "bar"),
///     ("fizz", "buzz"),
/// ]);
///
/// assert_eq!(query.first("foo"), Some("bar"));
/// assert_eq!(query.first("fizz"), Some("buzz"));
/// assert!(query.first("nonexistent").is_none());
/// ```
/// ---
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct QueryString(Vec<QueryParameter>);

impl QueryString {
    /// Returns the first occurrence of a value for a given key, if it exists.
    ///
    /// ```rust
    /// # use reqmd_http::QueryString;
    /// let query = QueryString::from_iter([
    ///     ("foo", "bar"),
    ///     ("fizz", "buzz"),
    ///     ("foo", "rab"),
    /// ]);
    ///
    /// assert_eq!(query.first("foo"), Some("bar"));
    /// assert_eq!(query.first("fizz"), Some("buzz"));
    /// assert!(query.first("nonexistent").is_none());
    /// ```
    /// ---
    pub fn first(&self, key: &str) -> Option<&str> {
        self.values_for(key).next()
    }

    /// Provides an iterator over all values for a given key.
    ///
    /// ```rust
    /// # use reqmd_http::QueryString;
    /// let query = QueryString::from_iter([
    ///     ("foo", "bar"),
    ///     ("fizz", "buzz"),
    ///     ("foo", "rab"),
    /// ]);
    ///
    /// let foos = query.values_for("foo").collect::<Vec<_>>();
    /// assert_eq!(foos, vec!["bar", "rab"]);
    /// ```
    /// ---
    pub fn values_for(&self, key: &str) -> impl Iterator<Item = &str> {
        self.0
            .iter()
            .filter(move |param| param.key.eq(key))
            .map(|param| param.value.as_str())
    }

    /// Provides a mutable iterator over all values for a given key.
    ///
    /// ```rust
    /// # use reqmd_http::QueryString;
    /// let mut query = QueryString::from_iter([
    ///     ("foo", "bar"),
    ///     ("fizz", "buzz"),
    ///     ("foo", "rab"),
    /// ]);
    ///
    /// for foo in query.values_for_mut("foo") {
    ///     if foo != "bar" {
    ///         foo.make_ascii_uppercase();
    ///     }
    /// }
    ///
    /// let foos = query.values_for("foo").collect::<Vec<_>>();
    /// assert_eq!(foos, vec!["bar", "RAB"]);
    /// assert_eq!(query.first("fizz"), Some("buzz"));
    /// ```
    /// ---
    pub fn values_for_mut(
        &mut self,
        key: &str,
    ) -> impl Iterator<Item = &mut String> {
        self.0
            .iter_mut()
            .filter(|param| param.key.eq(key))
            .map(|param| &mut param.value)
    }

    /// Returns a mutable reference to the first value for
    /// a given key, if it exists.
    ///
    /// ```rust
    /// # use reqmd_http::QueryString;
    /// let mut query = QueryString::from_iter([
    ///     ("foo", "bar"),
    ///     ("fizz", "buzz"),
    ///     ("foo", "rab"),
    /// ]);
    ///
    /// if let Some(foo) = query.first_mut("foo") {
    ///     foo.make_ascii_uppercase();
    /// }
    /// let foos = query.values_for("foo").collect::<Vec<_>>();
    /// assert_eq!(foos, vec!["BAR", "rab"]);
    /// assert_eq!(query.first("fizz"), Some("buzz"));
    /// ```
    /// ---
    pub fn first_mut(&mut self, key: &str) -> Option<&mut String> {
        self.values_for_mut(key).next()
    }

    /// Deletes the first occurrence of a key and returns its value if found.
    ///
    /// ```rust
    /// # use reqmd_http::QueryString;
    /// let mut query = QueryString::from_iter([
    ///     ("foo", "bar"),
    ///     ("fizz", "buzz"),
    ///     ("foo", "rab"),
    /// ]);
    ///
    /// let maybe_foo = query.delete_first("foo");
    /// assert_eq!(maybe_foo.as_deref(), Some("bar"));
    /// assert_eq!(query.first("foo"), Some("rab"));
    /// assert_eq!(query.first("fizz"), Some("buzz"));
    /// ```
    /// ---
    pub fn delete_first(&mut self, key: &str) -> Option<String> {
        self.0
            .iter()
            .position(|param| param.key.eq(key))
            .map(|pos| self.0.remove(pos).value)
    }

    /// Deletes all occurrences of a key and returns their values.
    ///
    ///
    /// ```rust
    /// # use reqmd_http::QueryString;
    /// let mut query = QueryString::from_iter([
    ///     ("foo", "bar"),
    ///     ("fizz", "buzz"),
    ///     ("foo", "rab"),
    /// ]);
    ///
    /// let foos = query.delete_all("foo");
    /// assert_eq!(foos, vec!["bar".to_string(), "rab".to_string()]);
    /// assert_eq!(query.first("foo"), None);
    /// assert_eq!(query.first("fizz"), Some("buzz"));
    /// ```
    /// ---
    pub fn delete_all(&mut self, key: &str) -> Vec<String> {
        let mut deleted_values = Vec::with_capacity(4);
        self.0.retain_mut(|param| {
            if param.key.eq(key) {
                let mut deleted = String::new();
                std::mem::swap(&mut deleted, &mut param.value);
                deleted_values.push(deleted);
                false
            } else {
                true
            }
        });
        deleted_values
    }

    /// Adds a new key-value pair to the query.
    ///
    /// ```rust
    /// # use reqmd_http::QueryString;
    /// let mut query = QueryString::from_iter([("foo", "bar")]);
    ///
    /// query.add("fizz", "buzz");
    /// query.add("foo", "rab");
    ///
    /// let foos = query.values_for("foo").collect::<Vec<_>>();
    /// assert_eq!(query.first("foo"), Some("bar"));
    /// assert_eq!(query.first("fizz"), Some("buzz"));
    /// assert_eq!(foos, vec!["bar", "rab"]);
    /// ```
    /// ---
    pub fn add(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.0.push(QueryParameter::new(key, value));
    }

    /// adds many key-value pairs to the query.
    ///
    /// ```rust
    /// # use reqmd_http::QueryString;
    /// let mut query = QueryString::from_iter([("foo", "bar")]);
    ///
    /// query.insert_many([
    ///     ("fizz", "buzz"),
    ///     ("foo", "rab"),
    /// ]);
    ///
    /// let foos = query.values_for("foo").collect::<Vec<_>>();
    /// assert_eq!(query.first("foo"), Some("bar"));
    /// assert_eq!(query.first("fizz"), Some("buzz"));
    /// assert_eq!(foos, vec!["bar", "rab"]);
    /// ```
    /// ---
    pub fn insert_many<P, Q>(&mut self, params: P)
    where
        P: IntoIterator<Item = Q>,
        Q: Into<QueryParameter>,
    {
        self.0.extend(params.into_iter().map(Into::into));
    }

    /// Tests if the query is empty.
    ///
    /// ```rust
    /// # use reqmd_http::QueryString;
    /// let some = QueryString::from_iter([("foo", "bar")]);
    /// let none = QueryString::default();
    ///
    /// assert!(!some.is_empty());
    /// assert!(none.is_empty());
    /// ```
    /// ---
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Return the number of query parameters.
    ///
    /// ```rust
    /// # use reqmd_http::QueryString;
    /// let some = QueryString::from_iter([("foo", "bar")]);
    /// let none = QueryString::default();
    ///
    /// assert_eq!(none.len(), 0);
    /// assert_eq!(some.len(), 1);
    /// ```
    /// ---
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns an iterator over the query parameters.
    ///
    /// ```rust
    /// # use reqmd_http::QueryString;
    /// let query = QueryString::from_iter([
    ///     ("foo", "bar"),
    ///     ("fizz", "buzz"),
    ///     ("foo", "rab"),
    /// ]);
    ///
    /// let vals = query.iter().map(|p| p.value.as_str()).collect::<Vec<_>>();
    /// assert_eq!(vals, vec!["bar", "buzz", "rab"]);
    /// ```
    /// ---
    pub fn iter(&self) -> impl Iterator<Item = &QueryParameter> {
        self.0.iter()
    }

    /// Returns a mutable iterator over the query parameters.
    ///
    /// ```rust
    /// # use reqmd_http::QueryString;
    /// let mut query = QueryString::from_iter([
    ///     ("foo", "bar"),
    ///     ("fizz", "buzz"),
    ///     ("foo", "rab"),
    /// ]);
    ///
    /// for param in query.iter_mut() {
    ///     if param.key == "foo" {
    ///         param.key.make_ascii_uppercase();
    ///     } else {
    ///         param.value.make_ascii_uppercase();
    ///     }
    /// }
    ///
    /// let foos = query.values_for("FOO").collect::<Vec<_>>();
    ///
    /// assert!(query.first("foo").is_none());
    /// assert_eq!(query.first("fizz"), Some("BUZZ"));
    /// assert_eq!(foos, vec!["bar", "rab"]);
    /// ```
    /// ---
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

impl IntoIterator for QueryString {
    type Item = QueryParameter;
    type IntoIter = std::vec::IntoIter<QueryParameter>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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
