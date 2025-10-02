use super::prelude::*;

#[derive(Debug, Clone)]
pub struct QueryParamsTable<'a> {
    params: &'a http::QueryString,
}

impl<'a> QueryParamsTable<'a> {
    pub fn min_height(&self) -> u16 {
        if self.params.is_empty() {
            3
        } else {
            (self.params.len() as u16).saturating_add(3)
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct QueryParamsTableState {
    table_state: TableState,
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
        let display_border = Block::default()
            .title_alignment(Alignment::Center)
            .title("◀︎[ Query Params ]▶︎")
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let rows: Vec<Row> = self
            .params
            .iter()
            .enumerate()
            .map(|(i, param)| {
                Row::new([
                    Cell::from(
                        Line::from(param.key.as_str())
                            .right_aligned()
                            .yellow()
                            .bold(),
                    ),
                    Cell::from(Line::from(param.value.as_str()).bold()),
                ])
                .style(if i % 2 == 0 {
                    Style::default().bg(Color::DarkGray)
                } else {
                    Style::default().bg(Color::Rgb(50, 50, 50))
                })
            })
            .collect();

        let table = Table::new(rows, table_constraints(self.params))
            .header(header_row(self.params))
            .block(display_border)
            .column_spacing(1);

        StatefulWidget::render(&table, area, buf, &mut state.table_state);
    }
}

impl<'a> From<&'a http::QueryString> for QueryParamsTable<'a> {
    fn from(params: &'a http::QueryString) -> Self {
        Self { params }
    }
}

impl<'a> From<&'a http::Request> for QueryParamsTable<'a> {
    fn from(req: &'a http::Request) -> Self {
        Self { params: &req.query }
    }
}

fn header_row(query: &http::QueryString) -> Row<'static> {
    if query.is_empty() {
        Row::new([Cell::from("No Query Parameters").gray().on_black()])
    } else {
        Row::new([
            Cell::from(Line::from("🔑  ").right_aligned()),
            Cell::from(Line::from("  📝")),
        ])
        .yellow()
        .on_black()
    }
}

fn table_constraints(query: &http::QueryString) -> [Constraint; 2] {
    if query.is_empty() {
        [Constraint::Percentage(100), Constraint::Percentage(0)]
    } else {
        [Constraint::Length(30), Constraint::Fill(1)]
    }
}
