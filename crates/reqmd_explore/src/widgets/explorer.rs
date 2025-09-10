use crate::widgets::{RequestDetailBlock, RequestList, RequestListState};

use ::ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{StatefulWidget, Widget},
};
use ::reqmd_http as http;

#[derive(Debug, Clone, Default)]
pub struct Explorer<'a> {
    items: &'a [http::Request],
}

#[derive(Debug, Clone, Default)]
pub struct ExplorerState {
    list_state: RequestListState,
}

impl StatefulWidget for &Explorer<'_> {
    type State = ExplorerState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [list_region, detail_region] =
            Layout::vertical([Constraint::Percentage(25), Constraint::Percentage(75)]).areas(area);
        let list = RequestList::new(self.items);
        if let Some(selected_request) = list.selected(&state.list_state) {
            let detail = RequestDetailBlock::new(selected_request);
            detail.render(detail_region, buf);
        }
        list.render(list_region, buf, &mut state.list_state);
    }
}

impl<'a> Explorer<'a> {
    pub fn new(items: &'a [http::Request]) -> Self {
        Self { items }
    }

    pub fn next(&self, state: &mut ExplorerState) {
        if self.items.is_empty() {
            return;
        }

        match state.list_state.selected() {
            Some(idx) if idx >= self.items.len() - 1 => state.list_state.select(Some(0)),
            Some(idx) => state.list_state.select(Some(idx + 1)),
            None => state.list_state.select(Some(0)),
        }
    }

    pub fn prev(&self, state: &mut ExplorerState) {
        if self.items.is_empty() {
            return;
        }

        match state.list_state.selected() {
            Some(0) => state.list_state.select(Some(self.items.len() - 1)),
            Some(idx) => state.list_state.select(Some(idx - 1)),
            None => state.list_state.select(Some(0)),
        }
    }
}
