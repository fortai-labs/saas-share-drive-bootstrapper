mod cli;
mod error;
mod generator;
mod markdown_generator;
mod models;
mod validator;

use clap::Parser;
use cli::{Cli, Commands};
use error::Result;
use log::{info, warn};
use models::DirectoryStructure;
use std::fs;

fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();

    // Parse command line arguments
    let cli = Cli::parse();

    match cli.command {
        Commands::Create {
            schema,
            output,
            organization,
            skip_validation,
        } => {
            info!("Creating directory structure from schema: {:?}", schema);

            // Validate the schema unless skipped
            if !skip_validation {
                info!("Validating schema...");
                validator::validate_schema_file(&schema)?;
                println!("Schema validation successful");
                info!("Schema validation successful");
            } else {
                println!("Schema validation skipped");
                warn!("Schema validation skipped");
            }

            // Parse the schema
            let schema_content = fs::read_to_string(&schema)?;
            let structure: DirectoryStructure = serde_json::from_str(&schema_content)?;

            // Generate the directory structure
            info!("Generating directory structure at: {:?}", output);
            generator::generate_structure(&structure, &output, organization.as_deref())?;

            println!("Directory structure created successfully");
            info!("Directory structure created successfully");
        }

        Commands::Validate { schema } => {
            info!("Validating schema: {:?}", schema);

            validator::validate_schema_file(&schema)?;

            println!("Schema validation successful");
            info!("Schema validation successful");
        }

        Commands::Generate { output } => {
            info!("Generating sample schema at: {:?}", output);

            generator::generate_sample_schema(&output)?;

            println!("Sample schema generated successfully");
            info!("Sample schema generated successfully");
        }
    }

    Ok(())
}
