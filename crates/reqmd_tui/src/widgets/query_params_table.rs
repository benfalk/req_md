use super::prelude::*;
use crate::widgets::{KeyValueTable, KeyValueTableState};

#[derive(Debug, Clone)]
pub struct QueryParamsTable<'a> {
    kv_table: KeyValueTable<'a>,
}

impl<'a> QueryParamsTable<'a> {
    pub fn min_height(&self) -> u16 {
        self.kv_table.min_height()
    }
}

#[derive(Debug, Clone, Default)]
pub struct QueryParamsTableState {
    table_state: KeyValueTableState,
}

impl Widget for QueryParamsTable<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let mut state = QueryParamsTableState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}

impl StatefulWidget for QueryParamsTable<'_> {
    type State = QueryParamsTableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        StatefulWidget::render(self.kv_table, area, buf, &mut state.table_state);
    }
}

impl<'a> From<&'a http::QueryString> for QueryParamsTable<'a> {
    fn from(params: &'a http::QueryString) -> Self {
        Self {
            kv_table: KeyValueTable::new(
                " Query Params ",
                params
                    .iter()
                    .map(|param| (param.key.as_str(), param.value.as_str())),
            ),
        }
    }
}

impl<'a> From<&'a http::Request> for QueryParamsTable<'a> {
    fn from(req: &'a http::Request) -> Self {
        Self::from(&req.query)
    }
}
