use ::anyhow::{Context as _, Result};
use ::clap::Parser;
use ::reqmd_app::{ReqmdApp, commands};
use ::reqmd_core::{builtin_processors, builtin_providers};

use reqmd_cli::structs::{Options, SerializedList};

#[tokio::main]
async fn main() -> Result<()> {
    ::dotenvy::dotenv().ok();

    let options = Options::parse();
    let markdown = options.markdown().context("looking for markdown input")?;
    let reqmd = ReqmdApp::builder()
        .maybe_http_timeout(options.timeout_duration())
        .provider(builtin_providers::EnvProvider::default())
        .processor(builtin_processors::EnvVarExpansion::default())
        .processor(builtin_processors::YamlAsJson::default())
        .build();

    let requests = reqmd
        .run(commands::ParseRequests { markdown })
        .await
        .context("parsing markdown input")?;

    if options.list_requests() {
        let json = ::serde_json::to_string_pretty(&SerializedList(&requests))
            .context("converting requests to json")?;
        println!("{json}");
        return Ok(());
    }

    let request = if let Some(line_num) = options.file_line()? {
        requests
            .at_line(line_num)
            .with_context(|| format!("could not find request at line {line_num}"))?
    } else {
        requests
            .first()
            .context("could not find any requests in markdown")?
    };

    let response = reqmd
        .run(commands::SendMd { request })
        .await
        .context("sending request failed")?;

    if let Some(text) = response.body.text() {
        print!("{text}");
    } else {
        eprintln!("body is not plain text printable");
    }

    Ok(())
}
