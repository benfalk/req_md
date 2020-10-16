use url::{Url, Position};
use crate::req::Request;

const VALID_METHODS: &'static [&'static str] = &[
    "GET",
    "HEAD",
    "POST",
    "PUT",
    "DELETE",
    "PATCH",
];

pub fn parse_request(input: &str) -> Option<Request> {
    let mut lines = input.split("\n");
    let (method, uri) = find_req_line(&mut lines)?;
    let headers = collect_headers(&mut lines);
    let host = host_from_uri_or_headers(&uri, &headers)?;
    let body = slurp_up_body(&mut lines);

    Some(Request {
        method,
        uri,
        headers,
        body,
        host,
    })
}

fn find_req_line(lines: &mut std::str::Split<&str>) -> Option<(String, String)> {
    while let Some(line) = lines.next() {
        for method in VALID_METHODS {
            if line.starts_with(method) {
                if let Some((method, rest)) = line.split_once(" ") {
                    return Some((method.to_string(), rest.to_string()));
                }
            }
        }
    }

    None
}

fn collect_headers(lines: &mut std::str::Split<&str>) -> Vec<String> {
    let mut headers = vec![];
    while let Some(line) = lines.next() {
        if line.starts_with("```") || line.len() == 0 { break; }
        headers.push(line.to_string());
    }
    headers
}

fn slurp_up_body(lines: &mut std::str::Split<&str>) -> Option<String> {
    let mut body = String::new();
    let first_line = lines.next()?;

    if !first_line.starts_with("```") {
        body.push_str(&first_line);
    }

    while let Some(line) = lines.next() {
        if line.starts_with("```") { break; }
        body.push_str(&line);
    }

    if body.len() > 0 {
        Some(body)
    }
    else {
        None
    }
}

fn host_from_uri_or_headers(uri: &String, headers: &Vec<String>) -> Option<String> {
    if let Ok(url) = Url::parse(&uri) {
        return Some(url[..Position::BeforePath].to_string());
    }

    for header in headers {
        if header.to_lowercase().starts_with("host: ") {
            return Some(header[6..].to_string());
        }
    }

    None
}
