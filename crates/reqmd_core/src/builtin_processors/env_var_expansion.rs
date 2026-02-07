use crate::FactoryProcessor;
use ::reqmd_ast as ast;
use ::reqmd_http as http;
use ::std::env::VarError;

/// # Environment Variable Expansion Processor
///
/// This processor scans the HTTP request fields for environment
/// variable references in the form `$VAR_NAME` and replaces them
/// with values from the environment.  If there is no value found
/// for a given variable, it is left unchanged.
///
/// Fields that are scanned include:
/// - query string keys and values
/// - header keys and values
/// - request path
/// - request body (if text)
///
/// ## Notes
///
/// - Only alphanumeric characters and underscores are supported
/// - Fails if the environment variable cannot be accessed (e.g. invalid unicode)
/// - If a variable is not found, it is left unchanged
///
/// ---
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct EnvVarExpansion;

impl FactoryProcessor for EnvVarExpansion {
    fn name(&self) -> &str {
        "EnvVarExpander"
    }

    fn update_request(
        &self,
        _data: &ast::HttpData,
        request: &mut http::Request,
    ) -> Result<(), Box<dyn ::std::error::Error + Send + Sync>> {
        for header in request.headers.iter_mut() {
            if let Some(expanded) = expand_env_vars(&header.value)? {
                header.value = expanded;
            }

            if let Some(expaned) = expand_env_vars(&header.key)? {
                header.key = expaned;
            }
        }

        for param in request.query.iter_mut() {
            if let Some(expanded) = expand_env_vars(&param.value)? {
                param.value = expanded;
            }

            if let Some(expanded) = expand_env_vars(&param.key)? {
                param.key = expanded;
            }
        }

        if let Some(expanded) = expand_env_vars(request.path.as_str())? {
            request.path = http::Path::from(expanded);
        }

        if let Some(body) = request.body.text()
            && let Some(expanded) = expand_env_vars(body)?
        {
            request.body = http::RequestBody::Text(expanded);
        }

        Ok(())
    }
}

fn expand_env_vars(input: &str) -> Result<Option<String>, VarError> {
    let mut chars = input.chars().peekable();
    let mut result = String::with_capacity(input.len());
    let mut var_name = String::with_capacity(64);
    let mut expanded = false;

    while let Some(ch) = chars.next() {
        if ch == '$' {
            var_name.clear();

            while let Some(&next_ch) = chars.peek() {
                if next_ch.is_alphanumeric() || next_ch == '_' {
                    var_name.push(next_ch);
                    chars.next(); // Consume the character
                } else {
                    break;
                }
            }

            if var_name.is_empty() {
                result.push('$');
                continue;
            }

            match ::std::env::var(&var_name) {
                Ok(var_value) => {
                    expanded = true;
                    result.push_str(&var_value);
                }
                Err(VarError::NotPresent) => {
                    result.push('$');
                    result.push_str(&var_name);
                }
                Err(e) => return Err(e),
            }
        } else {
            result.push(ch);
        }
    }

    Ok(expanded.then_some(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::support::fixtures::sample_env_var_expansion_request_file as file;

    #[rstest::rstest]
    fn expansion_works() {
        unsafe {
            std::env::set_var("TEST_VAR", "expanded_value");
        }

        let input = "This is a $TEST_VAR in the string.";
        let expected = "This is a expanded_value in the string.";
        let output = expand_env_vars(input).unwrap();
        assert_eq!(output.as_deref(), Some(expected));

        let input_no_var = "This string has no variables.";
        let output_no_var = expand_env_vars(input_no_var).unwrap();
        assert!(output_no_var.is_none());

        let input_unknown_var = "This is an $UNKNOWN_VAR in the string.";
        let output_unknown_var = expand_env_vars(input_unknown_var).unwrap();
        assert!(output_unknown_var.is_none());
    }

    #[rstest::rstest]
    fn env_var_expansion_processor_works(file: crate::File) {
        unsafe {
            ::std::env::set_var("API_KEY", "abcdef12345");
            ::std::env::set_var("DEBUG_MODE", "trace");
            ::std::env::set_var("FIRST_NAME", "John");
            ::std::env::set_var("API_VERSION", "v2");
        }

        let mut factory = crate::Factory::default();
        factory.register_factory_processor(EnvVarExpansion::default());
        let requests = factory.build_requests(&file).unwrap();
        let req = &requests
            .first()
            .expect("At least one request expected")
            .request;

        assert_eq!(requests.len(), 1);
        assert_eq!(req.path.as_str(), "/api/v2/users");
        assert_eq!(req.headers.len(), 1);
        assert_eq!(req.headers.first("x-api-key"), Some("abcdef12345"));
        assert_eq!(req.query.len(), 1);
        assert_eq!(req.query.first("debug"), Some("trace"));
        assert_eq!(req.body.text(), Some(r#"{ "first_name": "John" }"#));
    }
}
