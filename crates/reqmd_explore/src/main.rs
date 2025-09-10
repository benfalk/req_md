use ::color_eyre::Result;
use ::reqmd_core::HttpGroup;
use ::reqmd_explore::App;

fn main() -> Result<()> {
    ::color_eyre::install()?;
    let reqs = simple_req();
    let mut app = App::from(reqs);
    let mut terminal = ::ratatui::init();
    let result = app.run(&mut terminal);
    ::ratatui::restore();
    result
}

pub fn simple_req() -> HttpGroup {
    ::ron::from_str(include_str!("./support/multi.req.ron")).unwrap()
}
