use super::prelude::*;
use crate::widgets::{
    AddressDetailLine, BodyViewer, HeadersTable, QueryParamsTable,
    ResourceDetailLine,
};

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

        let block_inner = display_border.inner(area);

        let details = Rect {
            x: 0,
            y: 0,
            width: block_inner.width,
            height: block_inner.height.saturating_sub(3),
        };

        let params_table = QueryParamsTable::from(self.request);
        let headers_table = HeadersTable::from(self.request);
        let body_viewer = BodyViewer::from(self.request);

        let [
            address_layout,
            resource_layout,
            headers_layout,
            query_params_layout,
            body_viewer_layout,
        ] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(headers_table.min_height()),
            Constraint::Length(params_table.min_height()),
            Constraint::Length(body_viewer.min_height()),
        ])
        .flex(ratatui::layout::Flex::Start)
        .areas(details);

        let mut view =
            ScrollView::new(Size::new(block_inner.width, block_inner.height));
        view.render_widget(AddressDetailLine::from(self.request), address_layout);
        view.render_widget(ResourceDetailLine::from(self.request), resource_layout);
        view.render_widget(params_table, query_params_layout);
        view.render_widget(headers_table, headers_layout);
        view.render_widget(body_viewer, body_viewer_layout);
        let mut state = ScrollViewState::default();
        view.render(block_inner, buf, &mut state);

        display_border.render(area, buf);
    }
}
