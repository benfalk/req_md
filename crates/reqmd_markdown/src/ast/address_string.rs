use ::reqmd_http as http;

/// # Address String AST
///
/// Simple structure to hold both the parsed address
/// and the original string representation.
///
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AddressString {
    address: http::Address,
    data: String,
}

impl AddressString {
    pub fn as_str(&self) -> &str {
        self.data.as_str()
    }

    pub fn scheme(&self) -> http::Scheme {
        self.address.scheme
    }

    pub fn port(&self) -> Option<u16> {
        self.address.port
    }

    pub fn host(&self) -> &http::Host {
        &self.address.host
    }

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

impl<'de> ::serde::de::Deserialize<'de> for AddressString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::de::Deserializer<'de>,
    {
        let data = String::deserialize(deserializer)?;
        let address = http::Address::parse(&data).map_err(::serde::de::Error::custom)?;
        Ok(AddressString { address, data })
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

    #[rstest::rstest]
    fn deserialize_from_str() {
        let string = "\"https://example.com:8080\"";
        let address_string: AddressString = ::serde_json::from_str(string).unwrap();
        assert_eq!(address_string.as_str(), "https://example.com:8080");
        assert_eq!(address_string.scheme(), http::Scheme::Https);
        assert_eq!(address_string.host().to_string(), "example.com");
        assert_eq!(address_string.port(), Some(8080));
    }
}
