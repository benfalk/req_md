use ::clap::{Parser, Subcommand};
use ::reqmd_app::{ReqmdApp, processors, providers};
use ::reqmd_cli::{Target, TimeoutDuration};
use ::std::path::PathBuf;

#[tokio::main]
async fn main() -> ::anyhow::Result<()> {
    ::dotenvy::dotenv().ok();
    let args = Cli::parse();
    let reqmd = ReqmdApp::builder()
        .provider(providers::EnvProvider::default())
        .processor(processors::EnvVarExpansion::default())
        .processor(processors::YamlAsJson::default())
        .build();

    match args.command {
        Command::List { file } => ::reqmd_cli::list_requests(&file, &reqmd).await?,
        Command::Dump { file } => ::reqmd_cli::dump_ast(&file, &reqmd).await?,
        Command::Send { target, timeout } => {
            ::reqmd_cli::send_request(&target, &timeout, &reqmd).await?
        }
    }

    Ok(())
}

#[derive(Debug, Parser, Clone)]
#[command(author = "Ben Falk <github.com/benfalk>")]
#[command(about = "Tool for sending HTTP requests defined in markdown files")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand, Clone)]
enum Command {
    /// Lists all of the requests found in order
    List {
        /// File to list requests from
        file: PathBuf,
    },
    /// Sends request from file to server
    Send {
        ///  Examples:
        ///
        ///  sample.md:first    ( sends the first request )
        ///
        ///  sample.md:last     ( sends last request )
        ///
        ///  sample.md:3        ( sends third request )
        ///
        ///  sample.md:line10   ( sends request found at line 10 )
        target: Target,

        /// examples are 50ms, 3sec, 5min, none
        #[clap(short, long, default_value = "none")]
        timeout: TimeoutDuration,
    },
    /// Outputs JSON representation of parsed requests
    Dump {
        /// File to dump AST of into json
        file: PathBuf,
    },
}
