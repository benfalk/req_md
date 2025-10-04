pub use address_detail_line::AddressDetailLine;
pub use body_viewer::BodyViewer;
pub use explorer::{Explorer, ExplorerState};
pub use headers_table::{HeadersTable, HeadersTableState};
pub use key_value_table::{KeyValueTable, KeyValueTableState};
pub use query_params_table::{QueryParamsTable, QueryParamsTableState};
pub use request_detail_block::RequestDetailBlock;
pub use request_list::{RequestList, RequestListState};
pub use resource_detail_line::ResourceDetailLine;

mod address_detail_line;
mod body_viewer;
mod explorer;
mod headers_table;
mod key_value_table;
mod query_params_table;
mod request_detail_block;
mod request_list;
mod resource_detail_line;

/// this prelude module is used to re-export commonly used items
/// for the above modules to reduce boilerplate in their imports
mod prelude {
    pub(super) use ::ratatui::{
        buffer::Buffer,
        layout::{Alignment, Constraint, Layout, Rect, Size},
        style::{Color, Style, Stylize},
        text::{Line, Span},
        widgets::{
            Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row,
            StatefulWidget, Table, TableState, Widget, Wrap,
        },
    };
    pub(super) use ::reqmd_http as http;
    pub(super) use ::tui_scrollview::{ScrollView, ScrollViewState};
}
