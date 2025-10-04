use super::prelude::*;
use crate::widgets::{KeyValueTable, KeyValueTableState};

#[derive(Debug, Clone)]
pub struct HeadersTable<'a> {
    kv_table: KeyValueTable<'a>,
}

#[derive(Debug, Clone, Default)]
pub struct HeadersTableState {
    table_state: KeyValueTableState,
}

impl<'a> HeadersTable<'a> {
    pub fn min_height(&self) -> u16 {
        self.kv_table.min_height()
    }
}

impl Widget for HeadersTable<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let mut state = HeadersTableState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}

impl StatefulWidget for HeadersTable<'_> {
    type State = HeadersTableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        StatefulWidget::render(self.kv_table, area, buf, &mut state.table_state);
    }
}

impl<'a> From<&'a http::Headers> for HeadersTable<'a> {
    fn from(headers: &'a http::Headers) -> Self {
        Self {
            kv_table: KeyValueTable::new(
                " Headers ",
                headers
                    .iter()
                    .map(|header| (header.key.as_str(), header.value.as_str())),
            ),
        }
    }
}

impl<'a> From<&'a http::Request> for HeadersTable<'a> {
    fn from(req: &'a http::Request) -> Self {
        Self::from(&req.headers)
    }
}
