use tokio::io::{self};
// use tokio::net::TcpStream;
// use reqwest::Client;
use clap::Parser;
use clap::Subcommand;

use anyhow::Error;

use tabled::{
    settings::{object::Rows, themes::Colorization, Color, Style},
    Table, Tabled,
};

// TODO: Rely on our server for the source of truth rather than the runtimelib
use runtimelib;

/** Runtime 🔄  */
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Commands to interact with runtimes
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Start {
        name: String,
    },
    /// List running runtimes
    Ps,
    /// Kill a specific runtime
    Kill {
        /// ID of the runtime to kill
        id: String,
    },
    Attach {
        id: String,
    }
    /* TODO: Start a REPL session
    // Run {
    //     /// The REPL to start (e.g., python3, node)
    //     repl: String,
    // },
     */
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start { name } => {
            create_instance(name).await?;
        }
        Commands::Ps => {
            list_instances().await?;
        }
        Commands::Attach { id } => {
            attach_instance(id).await?;
        }
        Commands::Kill { id } => {
            kill_instance(id).await?;
        } // Commands::Run { repl } => {
          //     start_repl(repl).await?;
          // }
    }

    Ok(())
}

async fn create_instance(name: String) -> Result<(), Error> {
    Err(Error::msg(format!("No runtime for: {}", name)))
}

async fn attach_instance(id: String) -> Result<(), Error> {
    let client = runtimelib::attach(id).await.map_err(Error::msg)?;
    client.listen().await;
    Ok(())
}

#[derive(Tabled)]
struct RuntimeDisplay {
    #[tabled(rename = "Kernel Name")]
    kernel_name: String,
    #[tabled(rename = "Language")]
    language: String,
    #[tabled(rename = "IP")]
    ip: String,
    #[tabled(rename = "Transport")]
    transport: String,
    #[tabled(rename = "Connection File")]
    connection_file: String,
    #[tabled(rename = "State")]
    state: String,
}

async fn list_instances() -> io::Result<()> {
    let runtimes = runtimelib::list_instances().await;

    let displays: Vec<RuntimeDisplay> = runtimes
        .into_iter()
        .map(|runtime| RuntimeDisplay {
            kernel_name: runtime.kernel_name.chars().take(15).collect(),
            ip: runtime.ip,
            transport: runtime.transport,
            connection_file: runtime.connection_file,
            state: runtime.state,
            language: runtime.kernel_info["language_info"]["name"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
        })
        .collect();

    if !displays.is_empty() {
        let table = Table::new(displays)
            .with(Style::markdown())
            .with(Colorization::exact([Color::BOLD], Rows::first()))
            .to_string();
        println!("{}", table);
    } else {
        println!("No Jupyter runtimes running.");
    }

    Ok(())
}

async fn kill_instance(id: String) -> io::Result<()> {
    println!("No runtime running with ID: {}", id);
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("Runtime with ID {} not found", id),
    ))
}
