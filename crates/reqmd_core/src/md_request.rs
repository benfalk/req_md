use ::reqmd_http as http;
use ::reqmd_markdown::ast;

/// # Markdown Request
///
/// Represents a single HTTP request defined within a Markdown file.
///
/// ---
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct MdRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub request: http::Request,
    pub original: Box<ast::HttpData>,
}
