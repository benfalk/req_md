use super::prelude::*;

#[derive(Debug, Clone)]
pub struct BodyViewer<'a> {
    body: &'a http::RequestBody,
    #[allow(dead_code)] // Needed for future content-type based rendering
    content_type: Option<&'a str>,
}

impl<'a> BodyViewer<'a> {
    pub fn min_height(&self) -> u16 {
        2 + match self.body {
            http::RequestBody::None => 1,
            http::RequestBody::Text(text) => text.lines().count() as u16,
            http::RequestBody::Binary(data) => {
                16 / (data.len() as u16 + 15) // Each line shows 16 bytes
            }
        }
    }
}

impl Widget for BodyViewer<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let mut state = ();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}

impl StatefulWidget for BodyViewer<'_> {
    type State = ();

    fn render(self, area: Rect, buf: &mut Buffer, _state: &mut Self::State)
    where
        Self: Sized,
    {
        let display_border = Block::default()
            .title_alignment(Alignment::Center)
            .title(" Body ")
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        match self.body {
            http::RequestBody::None => {
                let empty_msg = Paragraph::new("")
                    .style(Style::default().fg(Color::DarkGray))
                    .alignment(Alignment::Center)
                    .block(display_border);
                empty_msg.render(area, buf);
            }
            http::RequestBody::Text(text) => {
                let paragraph = Paragraph::new(text.as_str())
                    .style(Style::default().fg(Color::White))
                    .block(display_border)
                    .wrap(Wrap { trim: true });
                paragraph.render(area, buf);
            }
            http::RequestBody::Binary(data) => {
                let hex_view =
                    data.chunks(16)
                        .enumerate()
                        .map(|(i, chunk)| {
                            let hex_bytes: Vec<String> =
                                chunk.iter().map(|b| format!("{:02X}", b)).collect();
                            let ascii_bytes: String = chunk
                                .iter()
                                .map(|&b| {
                                    if b.is_ascii_graphic() {
                                        b as char
                                    } else {
                                        '.'
                                    }
                                })
                                .collect();
                            format!(
                                "{:08X}: {:<48}  {}",
                                i * 16,
                                hex_bytes.join(" "),
                                ascii_bytes
                            )
                        })
                        .collect::<Vec<String>>()
                        .join("\n");

                let paragraph = Paragraph::new(hex_view)
                    .style(Style::default().fg(Color::White))
                    .block(display_border)
                    .wrap(Wrap { trim: false });
                paragraph.render(area, buf);
            }
        }
    }
}

impl<'a> From<&'a http::Request> for BodyViewer<'a> {
    fn from(request: &'a http::Request) -> Self {
        Self {
            body: &request.body,
            content_type: request.headers.first("Content-Type"),
        }
    }
}
