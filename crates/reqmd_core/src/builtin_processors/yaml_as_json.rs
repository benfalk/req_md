use crate::FactoryProcessor;
use ::reqmd_http as http;
use ::reqmd_markdown::ast;
use ::serde_json::Value as JsonValue;

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct YamlAsJsonProcessor;

impl FactoryProcessor for YamlAsJsonProcessor {
    fn name(&self) -> &str {
        "YamlAsJsonProcessor"
    }

    fn update_request(
        &self,
        data: &ast::HttpData,
        request: &mut http::Request,
    ) -> Result<(), Box<dyn ::std::error::Error + Send + Sync>> {
        if !(body_lang_is_yaml(data) && as_json_meta_found(data)) {
            return Ok(());
        }

        let Some(body_content) = data.body.content.text() else {
            return Ok(());
        };

        let json: JsonValue = ::serde_saphyr::from_str(body_content)?;
        let json_string = ::serde_json::to_string_pretty(&json)?;
        request.body = http::RequestBody::Text(json_string);
        Ok(())
    }
}

fn body_lang_is_yaml(data: &ast::HttpData) -> bool {
    matches!(
        data.body.lang.as_deref(),
        Some(lang) if lang.eq_ignore_ascii_case("yaml") || lang.eq_ignore_ascii_case("yml")
    )
}

fn as_json_meta_found(data: &ast::HttpData) -> bool {
    if let Some(meta) = &data.body.meta {
        meta.split_whitespace()
            .any(|token| token.eq_ignore_ascii_case("send-as-json"))
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::support::fixtures::sample_yaml_as_json_request_file as file;

    #[rstest::rstest]
    fn yaml_as_json_processor_works(file: crate::File) {
        let mut factory = crate::Factory::default();
        factory.register_factory_processor(YamlAsJsonProcessor::default());

        let requests = factory.build_requests(&file).unwrap();

        assert_eq!(requests.len(), 1);

        let body_text = requests
            .first()
            .and_then(|req| req.request.body.text())
            .expect("Request body should have text");

        assert_eq!(
            ::serde_json::from_str::<JsonValue>(body_text).unwrap(),
            ::serde_json::json!({
                "first_name": "John",
                "last_name": "Doe",
                "age": 22,
                "active": true,
                "hobbies": [
                    "reading",
                    "hiking",
                    "coding"
                ]
            })
        );
    }
}
