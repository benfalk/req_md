use ::reqmd_http as http;

/// # Address String AST
///
/// Simple structure to hold both the parsed address
/// and the original string representation.  It allows
/// for simple string serialization and deserialization
/// of addresses while also providing easy access to the
/// individual components of the address:
///
/// - [scheme]
/// - [host]
/// - [port]
///
/// [scheme]: AddressString::scheme
/// [host]: AddressString::host
/// [port]: AddressString::port
/// ---
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AddressString {
    address: http::Address,
    data: String,
}

impl AddressString {
    /// underlying string representation of the address
    ///
    /// ```rust
    /// # use reqmd_ast::AddressString;
    /// let address = "https://demo.com:8080".parse::<AddressString>().unwrap();
    /// assert_eq!(address.as_str(), "https://demo.com:8080");
    /// ```
    /// ---
    pub fn as_str(&self) -> &str {
        self.data.as_str()
    }

    /// scheme of the address
    ///
    /// ```rust
    /// # use reqmd_ast::AddressString;
    /// let address = "https://demo.com:8080".parse::<AddressString>().unwrap();
    /// assert_eq!(address.scheme(), reqmd_http::Scheme::Https);
    /// ```
    /// ---
    pub fn scheme(&self) -> http::Scheme {
        self.address.scheme
    }

    /// port of the address, if specified
    ///
    /// ```rust
    /// # use reqmd_ast::AddressString;
    /// let address = "https://demo.com:8080".parse::<AddressString>().unwrap();
    /// assert_eq!(address.port(), Some(8080));
    /// ```
    /// ---
    pub fn port(&self) -> Option<u16> {
        self.address.port
    }

    /// host parsed from the address string
    ///
    /// ```rust
    /// # use reqmd_ast::AddressString;
    /// let host = reqmd_http::Host::parse("demo.com").unwrap();
    /// let address = "https://demo.com:8080".parse::<AddressString>().unwrap();
    /// assert_eq!(address.host(), &host);
    /// ```
    /// ---
    pub fn host(&self) -> &http::Host {
        &self.address.host
    }

    /// full address data parsed from the string
    ///
    /// ```rust
    /// # use reqmd_ast::AddressString;
    /// let http_address = reqmd_http::Address::parse("http://d.c").unwrap();
    /// let address = "http://d.c".parse::<AddressString>().unwrap();
    /// assert_eq!(address.address(), &http_address);
    /// ```
    /// ---
    pub fn address(&self) -> &http::Address {
        &self.address
    }
}

impl From<AddressString> for http::Address {
    fn from(address_string: AddressString) -> Self {
        address_string.address
    }
}

impl From<AddressString> for String {
    fn from(address_string: AddressString) -> Self {
        address_string.data
    }
}

impl AsRef<http::Address> for AddressString {
    fn as_ref(&self) -> &http::Address {
        &self.address
    }
}

impl AsRef<str> for AddressString {
    fn as_ref(&self) -> &str {
        &self.data
    }
}

impl ::std::str::FromStr for AddressString {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let address = http::Address::parse(s).map_err(|err| {
            crate::Error::Parse(format!(
                "Error Parsing AddressString: {message}",
                message = err
            ))
        })?;
        Ok(AddressString {
            address,
            data: s.to_string(),
        })
    }
}

impl<'de> ::serde::de::Deserialize<'de> for AddressString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::de::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(::serde::de::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl ::serde::ser::Serialize for AddressString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::ser::Serializer,
    {
        serializer.serialize_str(&self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::googletest::prelude::*;

    #[rstest::fixture]
    fn string_value() -> &'static str {
        "https://example.com:8080"
    }

    #[rstest::fixture]
    fn address(string_value: &str) -> AddressString {
        string_value.parse().unwrap()
    }

    #[gtest]
    #[rstest::rstest]
    fn from_str(string_value: &str) {
        let address = string_value.parse::<AddressString>().unwrap();
        let expected_host = &http::Host::parse("example.com").unwrap();

        expect_that!(address.data, eq(string_value));
        expect_that!(address.address.scheme, eq(http::Scheme::Https));
        expect_that!(address.address.host, eq(expected_host));
        expect_that!(address.address.port, eq(Some(8080)));
    }

    #[rstest::rstest]
    fn deserialize(address: AddressString) {
        let json = ::serde_json::json!("https://example.com:8080");
        let address_string: AddressString = ::serde_json::from_value(json).unwrap();
        assert_eq!(address, address_string);
    }

    #[rstest::rstest]
    fn serialize(address: AddressString) {
        let json = ::serde_json::json!("https://example.com:8080");
        let serialized = ::serde_json::to_value(&address).unwrap();
        assert_eq!(serialized, json);
    }

    #[rstest::rstest]
    fn serialize_and_back(address: AddressString) {
        let serialized = ::serde_json::to_string(&address).unwrap();
        let deserialized: AddressString =
            ::serde_json::from_str(&serialized).unwrap();
        assert_eq!(address, deserialized);
    }
}
