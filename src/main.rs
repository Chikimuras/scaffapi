mod cli;
mod config;
mod generator;

use anyhow::Result;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    //Dispatch the command
    match &cli.command {
        cli::Command::New { name, stack } => {
            println!("Creating a new project: {} with stack: {:?}", name, stack);
            // TODO: generator::new_project(name, stack);
        }
        cli::Commands::Generate { kind, name } => {
            println!("Generating a new {}: {}", kind, name);
            // TODO: generator::generate(kind, name);
        }
        cli::Command::Config { key, value } => {
            if let Some(value) = value {
                println!("Setting config {} to {}", key, value);
                // TODO: config::set_config(key, value);
            }
            else {
                println!("Getting config for key: {}", key);
                // TODO: config::get_config(key);
            }
        }
    }
    Ok(())
}
