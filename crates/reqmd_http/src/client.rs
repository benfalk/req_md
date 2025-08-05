use crate::{error::Error, request::Request, response::Response};

#[cfg_attr(any(test, feature = "mock"), ::mockall::automock)]
pub trait Client {
    fn send(&self, request: &Request) -> impl Future<Output = Result<Response, Error>> + Send;
}

#[cfg(feature = "reqwest")]
mod reqwest_impl {
    use super::*;
    use crate::{header::Headers, request::Method, response::Status};
    use ::reqwest::Client as ReqwestClient;

    impl Client for ReqwestClient {
        async fn send(&self, request: &Request) -> Result<Response, Error> {
            let mut builder = self.request(
                match request.method {
                    Method::Get => ::reqwest::Method::GET,
                    Method::Post => ::reqwest::Method::POST,
                    Method::Put => ::reqwest::Method::PUT,
                    Method::Delete => ::reqwest::Method::DELETE,
                    Method::Patch => ::reqwest::Method::PATCH,
                    Method::Head => ::reqwest::Method::HEAD,
                    Method::Options => ::reqwest::Method::OPTIONS,
                    Method::Trace => ::reqwest::Method::TRACE,
                    Method::Connect => ::reqwest::Method::CONNECT,
                },
                request.url.clone(),
            );

            for header in &request.headers {
                builder = builder.header(&header.key, &header.value);
            }

            match &request.body {
                crate::request::Body::None => {}
                crate::request::Body::Text(text) => {
                    builder = builder.body(text.clone());
                }
                crate::request::Body::Binary(binary) => {
                    builder = builder.body(binary.clone());
                }
            };

            let reqwest_response = builder
                .send()
                .await
                .map_err(|e| Error::ClientError(Box::new(e)))?;

            let status = Status(reqwest_response.status().as_u16());

            let headers = reqwest_response.headers().iter().try_fold(
                Headers::default(),
                |mut acc, (k, v)| {
                    acc.add(
                        k.as_str(),
                        v.to_str().map_err(|e| Error::ClientError(Box::new(e)))?,
                    );
                    Ok::<_, Error>(acc)
                },
            )?;

            let bytes = reqwest_response
                .bytes()
                .await
                .map_err(|e| Error::ClientError(Box::new(e)))?;

            let body = if bytes.is_empty() {
                crate::response::Body::None
            } else {
                String::from_utf8(bytes.to_vec())
                    .map(crate::response::Body::Text)
                    .unwrap_or_else(|e| crate::response::Body::Binary(e.into_bytes()))
            };

            Ok(Response {
                status,
                headers,
                body,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::header::Headers;
    use crate::response::Status;

    fn create_blog() -> Request {
        Request::post("https://example.com/blog")
            .unwrap()
            .header("Content-Type", "application/json")
            .text_body(r#"{"title": "Hello", "content": "World!"}"#)
            .build()
    }

    #[tokio::test]
    async fn mocking_contract() {
        let mut mock = MockClient::new();
        mock.expect_send()
            .withf(move |req| *req == create_blog())
            .returning(|_| {
                Box::pin(async move {
                    Ok(Response {
                        status: Status(201),
                        headers: Headers::from_iter([("content-type", "text/plain")]),
                        body: crate::response::Body::Text("Created".to_string()),
                    })
                })
            });

        let response = mock.send(&create_blog()).await.unwrap();
        assert_eq!(response.status.0, 201);
        assert_eq!(
            response.headers.first_value_for("content-type"),
            Some("text/plain")
        );
        assert_eq!(
            response.body,
            crate::response::Body::Text("Created".to_string())
        );
    }
}
