use clap::{Command, IntoApp, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use imgurs::ImgurClient;
use std::io::{self, stdout};

use crate::cli::{album_info::*, credits::*, delete_image::*, info_image::*, upload_image::*};

// get version from Cargo.toml
const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[clap(
    name = "imgurs",
    about = "Imgur API CLI", long_about = None,
    version = VERSION.unwrap_or("unknown")
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about = "Print Client Rate Limit", display_order = 1)]
    Credits,

    #[clap(about = "Upload image to Imgur", display_order = 2)]
    Upload { path: String },

    #[clap(about = "Delete image from Imgur", display_order = 3)]
    Delete { delete_hash: String },

    #[clap(about = "Print image info", display_order = 4)]
    Info { id: String },

    #[clap(about = "Print album info", display_order = 5)]
    AlbumInfo { id: String },

    #[clap(
        about = "Generate completion file for a shell [bash, elvish, fish, powershell, zsh]",
        display_order = 6
    )]
    Completions { shell: Shell },

    #[clap(about = "Generate man page", display_order = 7)]
    Manpage,
}

fn print_completions<G: Generator>(gen: G, app: &mut Command) {
    generate(gen, app, app.get_name().to_string(), &mut stdout())
}

#[tokio::main]
pub async fn parse(client: ImgurClient) {
    let args = Cli::parse();

    match args.command {
        Commands::Credits => credits(client).await,

        Commands::Upload { path } => upload_image(client, path.to_string()).await,

        Commands::Delete { delete_hash } => delete_image(client, delete_hash.to_string()).await,

        Commands::Info { id } => image_info(client, id.to_string()).await,

        Commands::AlbumInfo { id } => album_info(client, id.to_string()).await,

        Commands::Completions { shell } => {
            let mut app = Cli::command();

            print_completions(shell, &mut app)
        }

        Commands::Manpage => {
            let clap_app = Cli::command();
            let man = clap_mangen::Man::new(clap_app);

            man.render(&mut io::stdout()).expect("generate manpage")
        }
    }
}
