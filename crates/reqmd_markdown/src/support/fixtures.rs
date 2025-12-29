use crate::parsing::ParseContext;

#[rstest::fixture]
pub fn post_widget_md() -> &'static str {
    include_str!("./fixtures/post-widgets.md")
}

#[rstest::fixture]
pub fn post_widget_parse_context(post_widget_md: &'static str) -> ParseContext<'static> {
    ParseContext::build(post_widget_md).unwrap()
}

#[rstest::fixture]
pub fn post_widget_http() -> &'static str {
    include_str!("./fixtures/post-widgets-http.txt")
}
