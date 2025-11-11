#[rstest::fixture]
pub fn post_widget_md() -> &'static str {
    include_str!("./fixtures/post-widgets.md")
}

#[rstest::fixture]
pub fn post_widget_mdast(post_widget_md: &str) -> ::markdown::mdast::Node {
    let options = &::markdown::ParseOptions {
        constructs: ::markdown::Constructs {
            frontmatter: true,
            ..Default::default()
        },
        ..Default::default()
    };
    ::markdown::to_mdast(post_widget_md, options).unwrap()
}

#[rstest::fixture]
pub fn post_widget_http() -> &'static str {
    include_str!("./fixtures/post-widgets-http.txt")
}
