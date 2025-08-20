use crate::application::TimeoutDuration;
use reqwest::Error;
use reqwest::blocking::{Client, RequestBuilder, Response};

mod meta;
pub use self::meta::Meta;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub uri: String,
    pub host: String,
    pub headers: Vec<String>,
    pub body: Option<String>,
    pub meta: Meta,
}

impl Request {
    pub fn send(&self) -> Result<Response, Error> {
        let mut builder = self.headers.iter().fold(self.builder(), |builder, header| {
            if let Some((key, val)) = header.split_once(": ") {
                if key.to_lowercase().starts_with("host") {
                    builder
                } else {
                    builder.header(key, val)
                }
            } else {
                builder
            }
        });

        // TODO: There has to be a more elegant way to do this
        builder = if let Some(TimeoutDuration { duration }) = self.meta.timeout {
            builder.timeout(duration)
        } else {
            builder
        };

        builder = if self.body.is_some() {
            builder.body(self.body.as_ref().unwrap().clone())
        } else {
            builder
        };

        builder.send()
    }

    // Private Functions

    fn builder(&self) -> RequestBuilder {
        let client = Client::new();
        let url = format!("{}{}", self.host, self.uri);

        match self.method.as_str() {
            "GET" => client.get(&url),
            "PUT" => client.put(&url),
            "POST" => client.post(&url),
            "DELETE" => client.delete(&url),
            "HEAD" => client.head(&url),
            "PATCH" => client.patch(&url),
            verb => panic!("{verb} is not a valid http verb!"),
        }
    }
}
