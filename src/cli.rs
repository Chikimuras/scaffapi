// src/cli.rs

use clap::{Parser, Subcommand, ValueEnum};

/// Scaffold API project generator
#[derive(Parser)]
#[command(name = "scaffapi")]
#[command(version = "0.1.0")]
#[command(author = "Alexandre Velia")]
#[command(about = "Generate API boilerplate code from templates", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new project from a template stack
    New {
        /// Name of the project
        name: String,

        /// Technology stack to use (e.g. fastapi, express, etc.)
        #[arg(short, long, value_enum)]
        stack: Stack,
    },

    /// Generate a component (model, router, etc.)
    Generate {
        /// Type of component to generate
        #[arg(value_enum)]
        kind: ComponentKind,

        /// Name of the component
        name: String,
    },

    /// Update a configuration value (saved globally)
    Config {
        /// Configuration key (e.g. author, license)
        key: String,

        /// Value to set for the key
        value: String,
    },
}

/// Available technology stacks
#[derive(Clone, Debug, ValueEnum)]
pub enum Stack {
    Fastapi,
    Express,
    Flask,
}

/// Types of components that can be generated
#[derive(Clone, Debug, ValueEnum)]
pub enum ComponentKind {
    Model,
    Router,
    Schema,
}