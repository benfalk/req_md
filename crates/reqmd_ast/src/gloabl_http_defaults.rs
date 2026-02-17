use super::AddressString;
use ::reqmd_http as http;

/// # Global HTTP Defaults
///
/// Provides default values for HTTP requests.  This
/// is normally populated from markdown frontmatter
/// under the `http` key.
///
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct GlobalHttpDefaults {
    pub server: AddressString,
    pub headers: http::Headers,
    pub query: http::QueryString,
}

impl GlobalHttpDefaults {
    /// create a [request factory] from the global defaults
    ///
    /// ```rust
    /// # use reqmd_ast::GlobalHttpDefaults;
    /// use ::reqmd_http as http;
    ///
    /// let defaults = GlobalHttpDefaults {
    ///     server: "https://api.example.com".parse().unwrap(),
    ///     headers: http::Headers::from_iter([("foo", "bar")]),
    ///     query: http::QueryString::from_iter([("baz", "qux")]),
    /// };
    ///
    /// let request = defaults
    ///     .factory()
    ///     .post("/v1/resource")
    ///     .build();
    ///
    /// assert_eq!(
    ///     request.build_url().as_str(),
    ///     "https://api.example.com/v1/resource?baz=qux"
    /// );
    /// assert_eq!(request.method, http::Method::Post);
    /// assert_eq!(request.headers.first("foo"), Some("bar"));
    /// ```
    ///
    /// [request factory]: http::RequestFactory
    /// ---
    pub fn factory(&self) -> http::RequestFactory {
        http::Request::factory()
            .address(self.server.address().clone())
            .with_headers(self.headers.clone())
            .with_query_string(self.query.clone())
            .build()
    }
}

// Support for non-serde deserialization of global
// HTTP defaults from markdown frontmatter.
mod serde_deserialize {
    use super::*;
    use ::serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
    use std::{borrow::Cow, fmt};

    struct KeyVal {
        key: String,
        value: String,
    }

    impl KeyVal {
        fn into_tuple(self) -> (String, String) {
            (self.key, self.value)
        }
    }

    impl<'de> Deserialize<'de> for KeyVal {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_map(KeyValVisitor)
        }
    }

    struct KeyValVisitor;

    impl<'de> Visitor<'de> for KeyValVisitor {
        type Value = KeyVal;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a key-value pair")
        }

        fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
        where
            V: MapAccess<'de>,
        {
            let mut opt_key = None;
            let mut opt_value = None;

            while let Some(map_key) = map.next_key::<Cow<'_, str>>()? {
                match map_key.as_ref() {
                    "key" => {
                        if opt_key.is_some() {
                            return Err(de::Error::duplicate_field("key"));
                        }
                        opt_key = Some(map.next_value()?);
                    }
                    "value" => {
                        if opt_value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        opt_value = Some(map.next_value()?);
                    }
                    unknown => {
                        return Err(de::Error::unknown_field(
                            unknown,
                            &["key", "value"],
                        ));
                    }
                }
            }

            let Some(key) = opt_key else {
                return Err(de::Error::missing_field("key"));
            };

            let Some(value) = opt_value else {
                return Err(de::Error::missing_field("value"));
            };

            Ok(KeyVal { key, value })
        }
    }

    type KeyValuePairs = Vec<KeyVal>;

    struct GlobalHttpDefaultsVisitor;

    impl<'de> Visitor<'de> for GlobalHttpDefaultsVisitor {
        type Value = GlobalHttpDefaults;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map representing global HTTP defaults")
        }

        fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
        where
            V: MapAccess<'de>,
        {
            let mut server = None;
            let mut headers = None;
            let mut query = None;

            while let Some(key) = map.next_key::<Cow<'_, str>>()? {
                match key.as_ref() {
                    "server" => {
                        if server.is_some() {
                            return Err(de::Error::duplicate_field("server"));
                        }
                        server = Some(map.next_value()?);
                    }
                    "headers" => {
                        if headers.is_some() {
                            return Err(de::Error::duplicate_field("headers"));
                        }
                        headers = Some(map.next_value::<KeyValuePairs>()?);
                    }
                    "query" => {
                        if query.is_some() {
                            return Err(de::Error::duplicate_field("query"));
                        }
                        query = Some(map.next_value::<KeyValuePairs>()?);
                    }
                    unknown => {
                        return Err(de::Error::unknown_field(
                            unknown,
                            &["server", "headers", "query"],
                        ));
                    }
                }
            }

            Ok(GlobalHttpDefaults {
                server: server.unwrap_or_default(),
                headers: headers
                    .unwrap_or_default()
                    .into_iter()
                    .map(KeyVal::into_tuple)
                    .collect(),
                query: query
                    .unwrap_or_default()
                    .into_iter()
                    .map(KeyVal::into_tuple)
                    .collect(),
            })
        }
    }

    impl<'de> Deserialize<'de> for GlobalHttpDefaults {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_map(GlobalHttpDefaultsVisitor)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::fixture]
    fn defaults() -> GlobalHttpDefaults {
        GlobalHttpDefaults {
            server: "https://api.example.com".parse().unwrap(),
            headers: http::Headers::from_iter([("foo", "bar")]),
            query: http::QueryString::from_iter([("baz", "qux")]),
        }
    }

    #[rstest::rstest]
    fn deserialize_works(defaults: GlobalHttpDefaults) {
        let example = ::serde_json::from_value(serde_json::json!({
            "server": "https://api.example.com",
            "headers": [
                { "key": "foo", "value": "bar" }
            ],
            "query": [
                { "key": "baz", "value": "qux" }
            ]
        }))
        .unwrap();

        assert_eq!(defaults, example);
    }

    #[rstest::rstest]
    fn serialize_and_back(defaults: GlobalHttpDefaults) {
        let serialized = serde_json::to_string(&defaults).unwrap();
        let deserialized = serde_json::from_str(&serialized).unwrap();
        assert_eq!(defaults, deserialized);
    }
}
