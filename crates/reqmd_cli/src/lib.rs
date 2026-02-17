use ::anyhow::{Context as _, Result};
use ::reqmd_app::{ReqmdApp, commands};
use ::reqmd_core::{MdRequest, MdRequestList};
use ::std::{borrow::Cow, path::PathBuf};

mod structs;

//=============================================================================
//=========================== Public Interface ================================
//=============================================================================

pub use structs::{Selection, Target, TimeoutDuration};

/// # List Requests
///
/// Prints a numbered list of all requests found in the
/// specified markdown file.  The title of each request
/// is either the markdown title attribute or the request
/// line for the request (e.g. `GET /api/users?active=true`).
///
/// ---
pub async fn list_requests(file: &PathBuf, app: &ReqmdApp) -> Result<()> {
    let md_requests = parse_requests(file, app).await?;

    if md_requests.is_empty() {
        return Ok(());
    }

    let padding = md_requests.len().ilog10() as usize + 1;

    for (i, md) in md_requests.iter().enumerate() {
        let nth = i + 1;
        let title = extract_title(md);
        println!("{nth:>padding$}. {title}");
    }
    Ok(())
}

/// # Dump AST
///
/// Parses the specified markdown file and prints the resulting
/// rquest AST as a pretty-printed JSON string.  This is useful
/// for debugging and third party tools that want to parse the
/// markdown file in the same way for a different purpose than
/// sending requests (e.g. generating API documentation).
///
/// ---
pub async fn dump_ast(file: &PathBuf, app: &ReqmdApp) -> Result<()> {
    let md_requests = parse_requests(file, app).await?;
    let json_string = ::serde_json::to_string_pretty(&md_requests)
        .context("converting to json")?;
    println!("{json_string}");
    Ok(())
}

/// # Send Request
pub async fn send_request(
    target: &Target,
    timeout: &TimeoutDuration,
    app: &ReqmdApp,
) -> Result<()> {
    let md_requests = parse_requests(&target.file, app).await?;

    let Some(request) = target.selection.select(&md_requests) else {
        ::anyhow::bail!("{:?} not found", target.selection);
    };

    let send_request = app.run(commands::SendMd { request });

    let result = match timeout.duration {
        Some(duration) => ::tokio::time::timeout(duration, send_request)
            .await
            .with_context(|| format!("timed out after {duration:?}"))?,
        None => send_request.await,
    };

    let response = result.context("sending request")?;

    if let Some(string) = response.body.text() {
        print!("{string}");
    } else {
        eprintln!("Binary Response!");
    }

    Ok(())
}

//=============================================================================
//=========================== Helper Functions ================================
//=============================================================================

async fn parse_requests(file: &PathBuf, app: &ReqmdApp) -> Result<MdRequestList> {
    let markdown = ::tokio::fs::read_to_string(file)
        .await
        .with_context(|| format!("Unable to read file: {file:?}"))?;

    app.run(commands::ParseRequests { markdown })
        .await
        .with_context(|| format!("Unable to parse requests from file: {file:?}"))
}

fn extract_title(md_req: &MdRequest) -> Cow<'_, str> {
    use std::fmt::Write;
    struct RequestLineWriter<'a>(&'a MdRequest);

    impl ::std::fmt::Display for RequestLineWriter<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let req = &self.0.data;
            f.write_str(req.method.as_str())?;
            f.write_char(' ')?;
            f.write_str(req.path.as_str())?;

            let Some(first_pair) = req.query.iter().next() else {
                return Ok(());
            };

            f.write_char('?')?;
            f.write_str(&first_pair.key)?;
            f.write_char('=')?;
            f.write_str(&first_pair.value)?;

            for pair in req.query.iter().skip(1) {
                f.write_char('&')?;
                f.write_str(&pair.key)?;
                f.write_char('=')?;
                f.write_str(&pair.value)?;
            }

            Ok(())
        }
    }

    if let Some(title) = md_req.title.as_ref() {
        title.into()
    } else {
        format!("{}", RequestLineWriter(md_req)).into()
    }
}
