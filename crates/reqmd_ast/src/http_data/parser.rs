use super::HttpData;
use crate::Error;
use ::nom::branch::alt;
use ::nom::bytes::complete::{is_a, tag};
use ::nom::character::complete::{alpha1, alphanumeric1, newline, space1};
use ::nom::combinator::{opt, recognize};
use ::nom::multi::{fold_many0, many1_count};
use ::nom::sequence::{preceded, separated_pair, terminated};
use ::nom::{Finish, IResult, Parser};
use ::reqmd_http as http;

pub(super) fn parse(input: &str) -> Result<HttpData, Error> {
    let (_, request) = http_data(input)
        .finish()
        .map_err(|err| Error::Parse(format!("HttpData parsing error: {}", err)))?;
    Ok(request)
}

fn http_data(input: &str) -> IResult<&str, HttpData> {
    let (input, method) = http_method(input)?;
    let (input, _) = space1(input)?;
    let (input, path) = http_path(input)?;
    let (input, _) = opt(preceded(newline, space1)).parse(input)?;
    let (input, query) = opt(http_query_string).parse(input)?;
    let (input, _) = opt(newline).parse(input)?;
    let (input, headers) = opt(http_headers).parse(input)?;
    let query = query.unwrap_or_default();
    let headers = headers.unwrap_or_default();

    Ok((
        input,
        HttpData {
            method,
            path,
            query,
            headers,
            ..Default::default()
        },
    ))
}

fn http_method(input: &str) -> IResult<&str, http::Method> {
    alpha1
        .map_res(|s: &str| s.parse::<http::Method>())
        .parse(input)
}

fn http_path(input: &str) -> IResult<&str, http::Path> {
    recognize(many1_count(alt((
        alphanumeric1,
        is_a("-._~!$&'()*+,;=:/@"),
    ))))
    .map(http::Path::from)
    .parse(input)
}

fn http_query_string(input: &str) -> IResult<&str, http::QueryString> {
    fn char_group(input: &str) -> IResult<&str, String> {
        recognize(many1_count(alt((
            alphanumeric1,
            is_a("@!\"'$%^*_-+()<>[]{}/|;`."),
            preceded(
                tag("\\"),
                alt((tag(" "), tag("="), tag("&"), tag("?"), tag("\\"))),
            ),
        ))))
        .map(String::from)
        .parse(input)
    }

    fn target_value(input: &str) -> IResult<&str, String> {
        alt((
            terminated(char_group, tag("&")),
            terminated(char_group, preceded(newline, preceded(space1, tag("&")))),
            char_group,
        ))
        .parse(input)
    }

    preceded(
        tag("?"),
        fold_many0(
            separated_pair(char_group, tag("="), target_value),
            http::QueryString::default,
            |mut query_string, (left, right)| {
                query_string.add(left, right);
                query_string
            },
        ),
    )
    .parse(input)
}

fn http_headers(input: &str) -> IResult<&str, http::Headers> {
    fn header_line(input: &str) -> IResult<&str, (&str, &str)> {
        separated_pair(
            recognize(many1_count(alt((alphanumeric1, is_a("-_"))))),
            tag(": "),
            recognize(many1_count(alt((
                alphanumeric1,
                is_a(" @!:\"#$%^&*()_-+={}[]|;'<>,.?/`~"),
            )))),
        )
        .parse(input)
    }

    fold_many0(
        alt((terminated(header_line, newline), header_line)),
        http::Headers::default,
        |mut headers, (key, value)| {
            headers.add(key, value);
            headers
        },
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    fn parse_method_verb() {
        let result = http_method("POST /api/v1/widgets").finish();
        let (rest, verb) = result.unwrap();
        assert_eq!(verb, http::Method::Post);
        assert_eq!(rest, " /api/v1/widgets");
    }

    #[rstest::rstest]
    fn parse_path() {
        let result = http_path("/api/v1/widgets?rofl=copter").finish();
        let (rest, path) = result.unwrap();
        assert_eq!(path.as_str(), "/api/v1/widgets");
        assert_eq!(rest, "?rofl=copter");
    }

    #[rstest::rstest]
    fn parse_path_with_special_characters() {
        let result = http_path("/api/v1/widg-ets._~!$&'()*+,;=:/@").finish();
        let (rest, path) = result.unwrap();
        assert_eq!(path.as_str(), "/api/v1/widg-ets._~!$&'()*+,;=:/@");
        assert_eq!(rest, "");
    }

    #[rstest::rstest]
    fn single_line_query_string() {
        let result = http_query_string("?foo=bar").finish();
        let (rest, query) = result.unwrap();
        assert_eq!(query.first("foo"), Some("bar"));
        assert_eq!("", rest);
    }

    #[rstest::rstest]
    fn query_string_with_dollar_signs() {
        let result = http_query_string("?price=$19.99&$ENV=true").finish();
        let (rest, query) = result.unwrap();
        assert_eq!(query.first("price"), Some("$19.99"));
        assert_eq!(query.first("$ENV"), Some("true"));
        assert_eq!("", rest);
    }

    #[rstest::rstest]
    fn multi_line_query_string() {
        let input = r#"?foo=bar
                       &biz=baz
                       &rofl=copter!"#;
        let result = http_query_string(input).finish();
        let (rest, query) = result.unwrap();
        assert_eq!(query.first("foo"), Some("bar"));
        assert_eq!(query.first("biz"), Some("baz"));
        assert_eq!(query.first("rofl"), Some("copter!"));
        assert_eq!("", rest);
    }

    #[rstest::rstest]
    fn single_line_header() {
        let input = "Content-Type: application/json";
        let result = http_headers(input).finish();
        let (rest, headers) = result.unwrap();
        assert_eq!(headers.first("Content-Type"), Some("application/json"));
        assert_eq!("", rest);
    }

    #[rstest::rstest]
    fn header_with_dollar_signs() {
        let input = "X-Api-Key: $API_KEY";
        let result = http_headers(input).finish();
        let (rest, headers) = result.unwrap();
        assert_eq!(headers.first("X-Api-Key"), Some("$API_KEY"));
        assert_eq!("", rest);
    }

    #[rstest::rstest]
    fn multi_line_headers() {
        let input = [
            "Content-Type: application/json",
            "Authorization: Bearer token",
        ]
        .join("\n");
        let result = http_headers(&input).finish();
        let (rest, headers) = result.unwrap();
        assert_eq!(headers.first("Content-Type"), Some("application/json"));
        assert_eq!(headers.first("Authorization"), Some("Bearer token"));
        assert_eq!(rest, "");
    }

    #[rstest::rstest]
    fn full_request_parsing_single_line_query_params() {
        let input = [
            "POST /api/v1/widgets?foo=bar&biz=baz",
            "Content-Type: application/json",
            "Authorization: Bearer token",
        ]
        .join("\n");

        let (rest, request) = http_data(&input).finish().unwrap();
        assert_eq!(request.method, http::Method::Post);
        assert_eq!(request.path.as_str(), "/api/v1/widgets");
        assert_eq!(request.query.first("foo"), Some("bar"));
        assert_eq!(request.query.first("biz"), Some("baz"));
        assert_eq!(request.headers.first("Authorization"), Some("Bearer token"));
        assert_eq!(
            request.headers.first("Content-Type"),
            Some("application/json")
        );
        assert_eq!(rest, "");
    }

    #[rstest::rstest]
    fn full_request_parsing_multi_line_query_params() {
        let input = [
            "GET /api/v1/widgets?foo=bar",
            "                   &biz=baz",
            "Content-Type: application/json",
            "Authorization: Bearer token",
        ]
        .join("\n");

        let (rest, request) = http_data(&input).finish().unwrap();
        assert_eq!(request.method, http::Method::Get);
        assert_eq!(request.path.as_str(), "/api/v1/widgets");
        assert_eq!(request.query.first("foo"), Some("bar"));
        assert_eq!(request.query.first("biz"), Some("baz"));
        assert_eq!(request.headers.first("Authorization"), Some("Bearer token"));
        assert_eq!(
            request.headers.first("Content-Type"),
            Some("application/json")
        );
        assert_eq!(rest, "");
    }

    #[rstest::rstest]
    fn full_request_parsing_multi_next_line_query_params() {
        let input = [
            "GET /api/v1/widgets",
            "    ?foo=bar",
            "    &biz=baz",
            "Content-Type: application/json",
            "Authorization: Bearer token",
        ]
        .join("\n");

        let (rest, request) = http_data(&input).finish().unwrap();
        assert_eq!(request.method, http::Method::Get);
        assert_eq!(request.path.as_str(), "/api/v1/widgets");
        assert_eq!(request.query.first("foo"), Some("bar"));
        assert_eq!(request.query.first("biz"), Some("baz"));
        assert_eq!(request.headers.first("Authorization"), Some("Bearer token"));
        assert_eq!(
            request.headers.first("Content-Type"),
            Some("application/json")
        );
        assert_eq!(rest, "");
    }

    #[rstest::rstest]
    fn full_request_parsing_no_headers() {
        let input = "DELETE /api/v1/widgets?foo=bar&biz=baz";
        let (rest, request) = http_data(input).finish().unwrap();
        assert_eq!(request.method, http::Method::Delete);
        assert_eq!(request.path.as_str(), "/api/v1/widgets");
        assert_eq!(request.query.first("foo"), Some("bar"));
        assert_eq!(request.query.first("biz"), Some("baz"));
        assert!(request.headers.is_empty());
        assert_eq!(rest, "");
    }

    #[rstest::rstest]
    fn full_request_parsing_no_query_params() {
        let input = [
            "PUT /api/v1/widgets",
            "Content-Type: application/json",
            "Authorization: Bearer token",
        ]
        .join("\n");
        let (rest, request) = http_data(&input).finish().unwrap();
        assert_eq!(rest, "");
        assert_eq!(request.method, http::Method::Put);
        assert_eq!(request.path.as_str(), "/api/v1/widgets");
        assert!(request.query.is_empty());
        assert_eq!(request.headers.first("Authorization"), Some("Bearer token"));
        assert_eq!(
            request.headers.first("Content-Type"),
            Some("application/json")
        );
    }

    #[rstest::rstest]
    fn full_request_parsing_no_query_params_or_header() {
        let input = "GET /api/v1/widgets";
        let (rest, request) = http_data(input).finish().unwrap();
        assert_eq!(request.method, http::Method::Get);
        assert_eq!(request.path.as_str(), "/api/v1/widgets");
        assert!(request.query.is_empty());
        assert!(request.headers.is_empty());
        assert_eq!(rest, "");
    }
}
