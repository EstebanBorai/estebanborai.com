use std::env::current_dir;

use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;

use libcli::{bookmarks, notes_index::NotesIndex};

#[derive(Debug, Parser)]
#[command(next_line_help = true)]
#[command(name = "website", author, version, about)]
pub enum Cli {
    /// Creates the Notes Entry Index
    Notes,
    /// Creates the Bookmarks Index
    Bookmarks {
        /// URL to bookmark
        url: Option<String>,
    },
}

impl Cli {
    pub async fn exec(self) -> Result<()> {
        match self {
            Self::Notes => {
                let index_path = current_dir()?.join("static").join("db");
                let index = NotesIndex::new()?;

                index.save_to_file(index_path)?;
            }
            Self::Bookmarks {
                url,
            } => {
                let bookmarks = bookmarks::BookmarksIndex::new()?;

                if let Some(url) = url {
                    let mut bookmarks = bookmarks;
                    return bookmarks.append(url).await;
                }

                bookmarks.list();
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
