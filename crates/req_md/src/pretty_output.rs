use reqwest::blocking::Response;
use std::fmt::Write;

pub trait PrettyOutput {
    fn pretty_output(self) -> String;
}

impl PrettyOutput for Response {
    fn pretty_output(self) -> String {
        let mut output = "```\n".to_owned();

        output
            .write_fmt(format_args!(
                "{} {}\n",
                self.status().as_u16(),
                self.status().canonical_reason().unwrap_or("")
            ))
            .unwrap();

        for (key, val) in self.headers().iter() {
            output
                .write_fmt(format_args!(
                    "{}: {}\n",
                    key.as_str(),
                    val.to_str().unwrap()
                ))
                .unwrap();
        }

        output.push_str("```\n");
        output.push_str("```");

        match self.as_ref() {
            &ContentType::Json => {
                output.push_str("json\n");
                let text = self.text().unwrap();

                match json::parse(&text) {
                    Ok(data) => output.push_str(&json::stringify_pretty(data, 2)),
                    Err(_) => output.push_str(&text),
                }
            }
            _ => {
                output.push('\n');
                output.push_str(self.text().unwrap().as_str());
            }
        }

        output.push_str("\n```");

        output
    }
}

enum ContentType {
    Json,
    Unknown,
}

impl AsRef<ContentType> for Response {
    fn as_ref(&self) -> &ContentType {
        let content_type = match self.headers().get("content-type") {
            Some(value) => value.to_str().unwrap(),
            None => return &ContentType::Unknown,
        };

        if content_type.contains("application/json") {
            return &ContentType::Json;
        }

        &ContentType::Unknown
    }
}
