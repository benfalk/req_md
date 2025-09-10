#![allow(dead_code, unused_imports, unused_variables)]

use ::ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};
use ::reqmd_http as http;

/// # Request Detail Block
///
/// Widget to display details of a selected HTTP request.
///
#[derive(Debug, Clone)]
pub struct RequestDetailBlock<'a> {
    request: &'a http::Request,
}

impl<'a> RequestDetailBlock<'a> {
    /// Create a new RequestDetailBlock for the given request.
    pub fn new(request: &'a http::Request) -> Self {
        Self { request }
    }
}

impl Widget for RequestDetailBlock<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let display_border = Block::default()
            .title("◀︎[ Details 🔍 ]▶︎")
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let mut lines = vec![host_line(self.request), request_line(self.request)];
        lines.extend(query_string_lines(&self.request.query));
        lines.extend(header_lines(&self.request.headers));
        Paragraph::new(lines)
            .block(display_border)
            .render(area, buf);
    }
}

fn blank_line() -> Line<'static> {
    Line::from(Span::raw(""))
}

fn host_line(req: &http::Request) -> Line<'_> {
    Line::from_iter([
        Span::styled("Host: ", Style::default().fg(Color::LightBlue)),
        Span::raw(req.address.build_url().to_string()),
    ])
}

fn request_line(req: &http::Request) -> Line<'_> {
    Line::from(Span::styled(
        format!("{} {}", req.method.as_str(), req.path.as_str()),
        Style::default().fg(Color::Yellow),
    ))
}

fn query_string_lines(params: &http::QueryString) -> Vec<Line<'_>> {
    if params.is_empty() {
        return vec![Line::from(Span::styled(
            "QueryString: (none)",
            Style::default().fg(Color::LightBlue),
        ))];
    }

    std::iter::once(Line::from(Span::styled(
        "QueryString:",
        Style::default().fg(Color::LightBlue),
    )))
    .chain(
        params
            .iter()
            .map(|p| Line::from(Span::raw(format!("  • {} = {}", p.key, p.value)))),
    )
    .collect()
}

fn header_lines(headers: &http::Headers) -> Vec<Line<'_>> {
    if headers.is_empty() {
        return vec![Line::from(Span::styled(
            "Headers: (none)",
            Style::default().fg(Color::LightBlue),
        ))];
    }

    std::iter::once(Line::from(Span::styled(
        "Headers:",
        Style::default().fg(Color::LightBlue),
    )))
    .chain(
        headers
            .iter()
            .map(|h| Line::from(Span::raw(format!("  • {}: {}", h.key, h.value)))),
    )
    .collect()
}
