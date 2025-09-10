use ::ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, List, ListState, StatefulWidget},
};
use ::reqmd_http as http;

/// # Request List
///
/// Widget to display a list of HTTP requests.
///
#[derive(Debug, Clone)]
pub struct RequestList<'a> {
    requests: &'a [http::Request],
}

#[derive(Debug, Clone, Default)]
pub struct RequestListState {
    list_state: ListState,
}

impl<'a> RequestList<'a> {
    pub fn new(requests: &'a [http::Request]) -> Self {
        Self { requests }
    }

    pub fn selected(&self, state: &RequestListState) -> Option<&'a http::Request> {
        state
            .list_state
            .selected()
            .and_then(|index| self.requests.get(index))
    }
}

impl RequestListState {
    pub fn new() -> Self {
        Self {
            list_state: ListState::default(),
        }
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.list_state.select(index);
    }

    pub fn selected(&self) -> Option<usize> {
        self.list_state.selected()
    }
}

impl StatefulWidget for RequestList<'_> {
    type State = RequestListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default()
            .title("◀︎[ Requests 📡 ]▶︎")
            .border_type(BorderType::Rounded)
            .borders(ratatui::widgets::Borders::ALL);

        let list =
            List::default()
                .highlight_symbol("➡️")
                .block(block)
                .items(self.requests.iter().map(|req| {
                    Line::default().spans([
                        ToSpan(req.method).into(),
                        Span::from(" "),
                        ToSpan(&req.path).into(),
                    ])
                }));

        list.render(area, buf, &mut state.list_state);
    }
}

struct ToSpan<T>(T);

impl<'a> From<ToSpan<&'a http::Path>> for Span<'a> {
    fn from(path: ToSpan<&'a http::Path>) -> Self {
        Span::styled(
            path.0.as_str(),
            Style::default().fg(Color::White).bg(Color::Black),
        )
    }
}

impl From<ToSpan<http::Method>> for Span<'static> {
    fn from(method: ToSpan<http::Method>) -> Self {
        match method.0 {
            http::Method::Get => Span::styled("GET", Style::default().fg(Color::Green)),
            http::Method::Post => Span::styled("POST", Style::default().fg(Color::Blue)),
            http::Method::Put => Span::styled("PUT", Style::default().fg(Color::Yellow)),
            http::Method::Delete => Span::styled("DELETE", Style::default().fg(Color::Red)),
            http::Method::Patch => Span::styled("PATCH", Style::default().fg(Color::Magenta)),
            http::Method::Head => Span::styled("HEAD", Style::default().fg(Color::Cyan)),
            http::Method::Connect => Span::styled("CONNECT", Style::default().fg(Color::White)),
            http::Method::Options => Span::styled("OPTIONS", Style::default().fg(Color::Gray)),
            http::Method::Trace => Span::styled("TRACE", Style::default().fg(Color::LightGreen)),
        }
    }
}
