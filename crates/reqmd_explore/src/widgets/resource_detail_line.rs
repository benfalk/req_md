use super::prelude::*;

#[derive(Debug, Clone)]
pub struct ResourceDetailLine<'a> {
    request: &'a http::Request,
}

impl<'a> ResourceDetailLine<'a> {
    fn build_line(&self) -> Line<'a> {
        const BLANK_SPACE: &str = "         ";

        let resource = self.request.path.as_str();
        let method = &self.request.method;
        let method_name = method.as_str();
        let opening_space = BLANK_SPACE.len() - method_name.len();
        let method_style = Style::new().bg(method_color(method)).fg(Color::Black);

        Line::from_iter([
            Span::raw(&BLANK_SPACE[0..opening_space]).style(method_style),
            Span::raw(method_name).style(method_style),
            Span::raw(" ").style(method_style),
            Span::raw(" ").style(method_style).reversed(),
            Span::raw(resource).style(method_style).reversed(),
            Span::raw(" ").style(method_style).reversed(),
        ])
    }
}

impl Widget for ResourceDetailLine<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        self.build_line().render(area, buf);
    }
}

fn method_color(method: &http::Method) -> Color {
    match method {
        http::Method::Get => Color::Green,
        http::Method::Post => Color::Blue,
        http::Method::Put => Color::Yellow,
        http::Method::Delete => Color::Red,
        http::Method::Patch => Color::Magenta,
        _ => Color::Gray,
    }
}

impl<'a> From<&'a http::Request> for ResourceDetailLine<'a> {
    fn from(request: &'a http::Request) -> Self {
        Self { request }
    }
}

impl<'a> From<ResourceDetailLine<'a>> for ListItem<'a> {
    fn from(value: ResourceDetailLine<'a>) -> Self {
        ListItem::from(value.build_line())
    }
}
