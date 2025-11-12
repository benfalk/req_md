use ::clap::Parser;
use ::color_eyre::Result;
use ::reqmd_explore::App;
use ::reqmd_http::Client as _;
use ::reqmd_markdown as markdown;

#[tokio::main]
async fn main() -> Result<()> {
    ::color_eyre::install()?;
    let document = Opts::parse().input()?;
    let mut app = App::from(document);
    let mut terminal = ::ratatui::init();
    let result = app.run(&mut terminal);
    ::ratatui::restore();
    if let Some(request) = result? {
        let resp = ::reqwest::Client::new().send(request).await?;
        let body = resp.body.text().unwrap_or("");
        println!("{body}");
    }
    Ok(())
}

#[derive(Parser, Debug)]
struct Opts {
    file: Option<String>,
}

impl Opts {
    fn input(&self) -> Result<markdown::ast::Document> {
        use std::io::Read;

        let data = match &self.file {
            Some(file_path) => std::fs::read_to_string(file_path)?,
            None => {
                let mut buffer = String::new();
                std::io::stdin().read_to_string(&mut buffer)?;
                buffer
            }
        };

        Ok(markdown::parse_markdown(&data)?)
    }
}
