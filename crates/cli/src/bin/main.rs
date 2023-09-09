use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;

use libcli::notes_index::NotesIndex;

#[derive(Debug, Parser)]
#[command(next_line_help = true)]
#[command(name = "website", author, version, about)]
pub enum Cli {
    /// Creates the Notes Entry Index
    Index,
}

impl Cli {
    pub async fn exec(self) -> Result<()> {
        match self {
            Self::Index => {
                let index = NotesIndex::new()?;
                println!("{:?}", index);
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let cli_opts = Cli::parse();

    cli_opts.exec().await.expect("Command Failed");
}
