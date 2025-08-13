use super::*;

#[rstest::rstest]
fn get_builder() {
    let req = Request::get("http://example.com")
        .expect("Failed to create GET request builder")
        .query_param("search", "markdown")
        .build();

    assert_eq!(req.method, Method::Get);
    assert_eq!(req.url.as_str(), "http://example.com/?search=markdown");
    assert!(req.headers.is_empty());
    assert_eq!(req.body, Body::None);
}

#[rstest::rstest]
fn post_builder() {
    let req = Request::post("http://example.com")
        .expect("Failed to create POST request builder")
        .header("Content-Type", "text/plain")
        .text_body("Hello, world!")
        .build();

    assert_eq!(req.method, Method::Post);
    assert_eq!(req.url.as_str(), "http://example.com/");
    assert_eq!(req.headers.len(), 1);
    assert_eq!(req.headers[0].key, "Content-Type");
    assert_eq!(req.headers[0].value, "text/plain");
    assert_eq!(req.body, Body::Text("Hello, world!".to_string()));
}

#[rstest::rstest]
fn put_builder() {
    let req = Request::put("http://example.com/resource")
        .expect("Failed to create PUT request builder")
        .header("Authorization", "Bearer token")
        .binary_body([42, 43, 44])
        .build();

    assert_eq!(req.method, Method::Put);
    assert_eq!(req.url.as_str(), "http://example.com/resource");
    assert_eq!(req.headers.len(), 1);
    assert_eq!(req.headers[0].key, "Authorization");
    assert_eq!(req.headers[0].value, "Bearer token");
    assert_eq!(req.body, Body::Binary(vec![42, 43, 44]));
}

#[rstest::rstest]
fn delete_builder() {
    let req = Request::delete("http://example.com/resource/123")
        .expect("Failed to create DELETE request builder")
        .build();

    assert_eq!(req.method, Method::Delete);
    assert_eq!(req.url.as_str(), "http://example.com/resource/123");
    assert!(req.headers.is_empty());
    assert_eq!(req.body, Body::None);
}
