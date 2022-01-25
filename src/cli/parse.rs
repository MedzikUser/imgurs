use clap::{AppSettings, Parser, Subcommand, App, IntoApp};
use clap_complete::{generate, Shell, Generator};
use log::error;

use std::io::stdout;

use imgurs::api::configuration::ImgurHandle;

use crate::cli::{credits::*, delete_image::*, info_image::*, upload_image::*};

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const NAME: Option<&str> = option_env!("CARGO_PKG_NAME");

#[derive(Parser, Debug)]
#[clap(
    name = NAME.unwrap_or("unknown"),
    about = "Imgur API CLI", long_about = None,
    version = VERSION.unwrap_or("unknown")
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about = "Print API Rate Limit")]
    Credits,

    #[clap(
        setting(AppSettings::ArgRequiredElseHelp),
        about = "Upload image to Imgur"
    )]
    Upload { path: String },

    #[clap(
        setting(AppSettings::ArgRequiredElseHelp),
        about = "Delete image from Imgur"
    )]
    Delete { delete_hash: String },

    #[clap(setting(AppSettings::ArgRequiredElseHelp), about = "Print image info")]
    Info { id: String },

    #[clap(setting(AppSettings::ArgRequiredElseHelp), about = "Print shell completions (bash, zsh, fish, powershell)")]
    Completions {
        shell: String,
    },
}

fn print_completions<G: Generator>(gen: G, app: &mut App) {
    generate(gen, app, app.get_name().to_string(), &mut stdout())
}

pub async fn parse(client: ImgurHandle) {
    let args = Cli::parse();

    match &args.command {
        Commands::Credits => {
            credits(client).await;
        }

        Commands::Upload { path } => {
            upload_image(client, path).await;
        }

        Commands::Delete { delete_hash } => {
            delete_image(client, delete_hash.to_string()).await;
        }

        Commands::Info { id } => {
            image_info(client, id).await;
        }

        Commands::Completions { shell } => {
            let mut app = Cli::into_app();

            if shell == "bash" {
                print_completions(Shell::Bash, &mut app);
            } else if shell == "zsh" {
                print_completions(Shell::Zsh, &mut app);
            } else if shell == "fish" {
                print_completions(Shell::Fish, &mut app);
            } else if shell == "powershell" {
                print_completions(Shell::PowerShell, &mut app);
            } else {
                error!("Completions to shell `{shell}`, not found!")
            }
        }
    }
}
