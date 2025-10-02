use super::prelude::*;

#[derive(Debug, Clone)]
pub struct HeadersTable<'a> {
    headers: &'a http::Headers,
}

#[derive(Debug, Clone, Default)]
pub struct HeadersTableState {
    table_state: TableState,
}

impl<'a> HeadersTable<'a> {
    pub fn min_height(&self) -> u16 {
        if self.headers.is_empty() {
            3
        } else {
            (self.headers.len() as u16).saturating_add(3)
        }
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
        let display_border = Block::default()
            .title_alignment(Alignment::Center)
            .title("◀︎[ Headers ]▶︎")
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let rows: Vec<Row> = self
            .headers
            .iter()
            .enumerate()
            .map(|(i, header)| {
                Row::new([
                    Cell::from(
                        Line::from(header.key.as_str())
                            .right_aligned()
                            .yellow()
                            .bold(),
                    ),
                    Cell::from(Line::from(header.value.as_str()).bold()),
                ])
                .style(if i % 2 == 0 {
                    Style::default().bg(Color::DarkGray)
                } else {
                    Style::default().bg(Color::Rgb(50, 50, 50))
                })
            })
            .collect();

        let table = Table::new(rows, table_constraints(self.headers))
            .header(header_row(self.headers))
            .block(display_border)
            .column_spacing(1);

        StatefulWidget::render(table, area, buf, &mut state.table_state);
    }
}

fn header_row(headers: &http::Headers) -> Row<'static> {
    if headers.is_empty() {
        Row::new([Cell::from("No Headers").gray().on_black()])
    } else {
        Row::new([
            Cell::from(Line::from("🔑  ").right_aligned()),
            Cell::from(Line::from("  📝")),
        ])
        .yellow()
        .on_black()
    }
}

fn table_constraints(headers: &http::Headers) -> [Constraint; 2] {
    if headers.is_empty() {
        [Constraint::Percentage(100), Constraint::Percentage(0)]
    } else {
        [Constraint::Length(30), Constraint::Fill(1)]
    }
}

impl<'a> From<&'a http::Headers> for HeadersTable<'a> {
    fn from(headers: &'a http::Headers) -> Self {
        Self { headers }
    }
}

impl<'a> From<&'a http::Request> for HeadersTable<'a> {
    fn from(req: &'a http::Request) -> Self {
        Self {
            headers: &req.headers,
        }
    }
}
