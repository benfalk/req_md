use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct KeyValueTable<'a> {
    title: &'a str,
    items: Vec<(&'a str, &'a str)>,
}

#[derive(Debug, Clone, Default)]
pub struct KeyValueTableState {
    state: TableState,
}

impl<'a> KeyValueTable<'a> {
    pub fn new<I>(title: &'a str, items: I) -> Self
    where
        I: Iterator<Item = (&'a str, &'a str)>,
    {
        Self {
            title,
            items: items.collect(),
        }
    }

    pub fn min_height(&self) -> u16 {
        self.items.len().saturating_add(3) as u16
    }

    fn table_constraints(&self) -> [Constraint; 2] {
        if self.items.is_empty() {
            [Constraint::Percentage(100), Constraint::Percentage(0)]
        } else {
            [
                Constraint::Length(self.max_key_length() + 2),
                Constraint::Fill(1),
            ]
        }
    }

    fn max_key_length(&self) -> u16 {
        self.items
            .iter()
            .map(|(key, _)| key.len())
            .max()
            .unwrap_or(0) as u16
    }

    fn rows(&self) -> Vec<Row<'_>> {
        self.items
            .iter()
            .enumerate()
            .map(|(i, (key, value))| {
                Row::new([
                    Cell::from(Line::from(*key).right_aligned().yellow().bold()),
                    Cell::from(Line::from(*value).bold()),
                ])
                .style(if i % 2 == 0 {
                    Style::default().bg(Color::DarkGray)
                } else {
                    Style::default().bg(Color::Rgb(50, 50, 50))
                })
            })
            .collect()
    }

    fn header_row(&self) -> Row<'_> {
        if self.items.is_empty() {
            Row::new([""])
        } else {
            Row::new([
                Cell::from(Line::from("🔑").centered()),
                Cell::from(Line::from("🗒")),
            ])
            .yellow()
            .on_black()
        }
    }

    fn border_style(&self) -> Style {
        if self.items.is_empty() {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default().fg(Color::White)
        }
    }
}

impl Widget for KeyValueTable<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let mut state = KeyValueTableState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}

impl StatefulWidget for KeyValueTable<'_> {
    type State = KeyValueTableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let display_border = Block::default()
            .title_alignment(Alignment::Center)
            .title(self.title)
            .style(self.border_style())
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let table = Table::new(self.rows(), self.table_constraints())
            .header(self.header_row())
            .block(display_border)
            .column_spacing(1);

        StatefulWidget::render(table, area, buf, &mut state.state);
    }
}
