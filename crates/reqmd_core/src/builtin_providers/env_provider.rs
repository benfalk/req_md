use crate::DefaultProvider;
use ::reqmd_ast as ast;
use ::std::{borrow::Cow, str::FromStr};

/// # Environment Variable Provider
///
/// Scans environment variables for global HTTP defaults using a
/// specified prefix ( defaults to `REQMD_` ).  Based on the suffix
/// after the prefix, the provider will set headers, query parameters,
/// and the server address.
///
/// ## Example
///
/// ```rust
/// # use ::reqmd_core::builtin_providers::EnvProvider;
/// # use ::reqmd_ast::GlobalHttpDefaults;
/// # use ::reqmd_core::DefaultProvider;
///
/// unsafe {
///     ::std::env::set_var("MYAPP_HEADER_X-API-KEY", "abcde");
///     ::std::env::set_var("MYAPP_QUERY_debug", "yes");
///     ::std::env::set_var("MYAPP_SERVER", "https://myapp.example.com");
/// }
/// let mut defaults = GlobalHttpDefaults::default();
/// let provider = EnvProvider::new("MYAPP_");
///
/// provider.apply_global_defaults(&mut defaults).unwrap();
///
/// assert_eq!(defaults.headers.first("x-api-key"), Some("abcde"));
/// assert_eq!(defaults.query.first("debug"), Some("yes"));
/// assert_eq!(defaults.server.as_str(), "https://myapp.example.com");
/// ```
/// ---
#[derive(Debug, Clone)]
pub struct EnvProvider {
    prefix: Cow<'static, str>,
}

impl Default for EnvProvider {
    fn default() -> Self {
        Self {
            prefix: Cow::Borrowed("REQMD_"),
        }
    }
}

impl EnvProvider {
    pub fn new(prefix: impl Into<Cow<'static, str>>) -> Self {
        Self {
            prefix: prefix.into(),
        }
    }
}

impl DefaultProvider for EnvProvider {
    fn name(&self) -> &str {
        "EnvProvider"
    }

    fn apply_global_defaults(
        &self,
        defaults: &mut ast::GlobalHttpDefaults,
    ) -> Result<(), Box<dyn ::std::error::Error + Send + Sync>> {
        for (key, value) in ::std::env::vars() {
            let Some(("", key_str)) = key.split_once(self.prefix.as_ref()) else {
                continue;
            };

            match key_str {
                key if key.starts_with("HEADER_") => {
                    let header_name = &key["HEADER_".len()..];
                    defaults.headers.add(header_name, value.as_str());
                }
                key if key.starts_with("QUERY_") => {
                    let query_name = &key["QUERY_".len()..];
                    defaults.query.add(query_name, value.as_str());
                }
                "SERVER" => {
                    defaults.server = ast::AddressString::from_str(value.as_str())?;
                }
                _ => continue,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    fn default_parsing() {
        unsafe {
            ::std::env::set_var("REQMD_HEADER_X-API-KEY", "12345");
            ::std::env::set_var("REQMD_QUERY_debug", "true");
            ::std::env::set_var("REQMD_SERVER", "https://api.example.com:8080");
        }

        let provider = EnvProvider::default();
        let mut defaults = ast::GlobalHttpDefaults::default();

        provider.apply_global_defaults(&mut defaults).unwrap();

        assert_eq!(defaults.headers.first("x-api-key"), Some("12345"));
        assert_eq!(defaults.query.first("debug"), Some("true"));
        assert_eq!(defaults.server.as_str(), "https://api.example.com:8080");
    }

    #[rstest::rstest]
    fn empty_prefix() {
        unsafe {
            ::std::env::set_var("HEADER_X-API-KEY", "67890");
            ::std::env::set_var("QUERY_debug", "false");
            ::std::env::set_var("SERVER", "http://localhost:3000");
        }

        let provider = EnvProvider::new("");
        let mut defaults = ast::GlobalHttpDefaults::default();

        provider.apply_global_defaults(&mut defaults).unwrap();

        assert_eq!(defaults.headers.first("x-api-key"), Some("67890"));
        assert_eq!(defaults.query.first("debug"), Some("false"));
        assert_eq!(defaults.server.as_str(), "http://localhost:3000");
    }
}
