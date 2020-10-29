use reqwest::blocking::{Client, RequestBuilder, Response};
use reqwest::Error;
mod meta;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub uri: String,
    pub host: String,
    pub headers: Vec<String>,
    pub body: Option<String>,
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
            verb => panic!("{} is not a valid http verb!", verb),
        }
    }
}
