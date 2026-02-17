use super::prelude::*;
use crate::widgets::ResourceDetailLine;

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

        let list = List::default()
            .highlight_symbol("➡️")
            .block(block)
            .items(self.requests.iter().map(ResourceDetailLine::from));

        StatefulWidget::render(list, area, buf, &mut state.list_state);
    }
}
