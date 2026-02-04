use crate::{error::Error, request::Request, response::Response};

/// # HTTP Client Trait
///
/// This crate does not define a specific HTTP client implementation.
/// Instead, it defines a trait `Client` that can be implemented for
/// any client that is needed, such as `reqwest`, `hyper`, or others.
///
/// Currently, the only implementation provided is for `reqwest`.
/// You'll need to enable the `reqwest` feature in your `Cargo.toml`
/// for it.
///
/// Also, the `Client` trait is designed to be mockable for testing
/// purposes.  If you enable the `mock` feature, it will provide a
/// [mockall] implementation that can be used in tests.
///
#[cfg_attr(any(test, feature = "mock"), ::mockall::automock)]
pub trait Client {
    fn send(
        &self,
        request: &Request,
    ) -> impl Future<Output = Result<Response, Error>> + Send;
}

#[cfg(feature = "reqwest")]
mod reqwest_impl {
    use super::*;
    use crate::{header::Headers, request::Method};
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
                request.build_url(),
            );

            for header in &request.headers {
                builder = builder.header(&header.key, &header.value);
            }

            match &request.body {
                crate::request::RequestBody::None => {}
                crate::request::RequestBody::Text(text) => {
                    builder = builder.body(text.clone());
                }
                crate::request::RequestBody::Binary(binary) => {
                    builder = builder.body(binary.clone());
                }
            };

            let reqwest_response = builder
                .send()
                .await
                .map_err(|e| Error::ClientError(Box::new(e)))?;

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

            let status = reqwest_response.status().as_u16();

            let bytes = reqwest_response
                .bytes()
                .await
                .map_err(|e| Error::ClientError(Box::new(e)))?;

            let body = if bytes.is_empty() {
                crate::response::ResponseBody::None
            } else {
                String::from_utf8(bytes.to_vec())
                    .map(crate::response::ResponseBody::Text)
                    .unwrap_or_else(|e| {
                        crate::response::ResponseBody::Binary(e.into_bytes())
                    })
            };

            Ok(Response::builder()
                .status(status)
                .multiple_headers(headers)
                .body(body)
                .build())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::address::Scheme;
    use crate::request::Method;

    fn create_blog() -> Request {
        Request::builder()
            .method(Method::Post)
            .address_scheme(Scheme::Https)
            .header("Content-Type", "application/json")
            .build()
    }

    #[tokio::test]
    async fn mocking_contract() {
        let mut mock = MockClient::new();
        mock.expect_send()
            .withf(move |req| *req == create_blog())
            .returning(|_| {
                Box::pin(async move {
                    Ok(Response::builder()
                        .status(201)
                        .header("content-type", "text/plain")
                        .body_text("Created")
                        .build())
                })
            });

        let response = mock.send(&create_blog()).await.unwrap();
        assert_eq!(response.status.0, 201);
        assert_eq!(response.headers.first("content-type"), Some("text/plain"));
        assert_eq!(
            response.body,
            crate::response::ResponseBody::Text("Created".to_string())
        );
    }
}
