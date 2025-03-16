use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "aidir",
    about = "Bootstrap AI-ready directory structures for startups and organizations",
    version,
    author
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a directory structure from a JSON schema
    Create {
        /// Path to the JSON schema file
        #[arg(short, long)]
        schema: PathBuf,

        /// Path to the output directory
        #[arg(short, long)]
        output: PathBuf,

        /// Organization name to use in README files
        #[arg(short = 'g', long)]
        organization: Option<String>,

        /// Skip validation of the JSON schema
        #[arg(long, default_value = "false")]
        skip_validation: bool,
    },

    /// Validate a JSON schema file against the schema definition
    Validate {
        /// Path to the JSON schema file
        #[arg(short, long)]
        schema: PathBuf,
    },

    /// Generate a sample JSON schema file
    Generate {
        /// Path to save the generated schema
        #[arg(short, long)]
        output: PathBuf,
    },
} 