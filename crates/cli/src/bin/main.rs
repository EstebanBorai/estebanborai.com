use clap::Parser;
use dotenv::dotenv;

#[derive(Debug, Parser)]
#[command(next_line_help = true)]
#[command(name = "website", author, version, about)]
pub enum Cli {
    /// Manage Blog
    Blog
}

impl Cli {
    pub async fn exec(self) {
        match self {
            Self::Blog => {
                println!("Blog command")
            },
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let cli_opts = Cli::parse();

    cli_opts.exec().await;
}
