mod categorization;
mod os;

use categorization::Categorization;
use os::OS;
use std::error::Error;

use clap::{Parser, Subcommand};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate Markdown files.
    Generate {
        // no arguments for now
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {} => generate(),
    }
}

fn generate() -> Result<()> {
    // Load categorization
    let categorization = Categorization::load("categorization.yml")?;

    // Configure renderer environment and common variables
    let mut env = minijinja::Environment::new();
    env.add_global(
        "auto_generated_disclaimer",
        "<!-- This file was auto-generated, please edit it's *.j2 template instead -->",
    );
    env.add_filter("repeat", |value: String, num: usize| value.repeat(num));
    env.set_keep_trailing_newline(true);

    // Load operating systems
    let oses: Vec<Box<dyn OS>> = vec![
        Box::new(os::Darwin::new()?),
        Box::new(os::FreeBSD::new()?),
        Box::new(os::Linux::new()?),
        Box::new(os::OpenBSD::new()?),
    ];

    // Render categorized system calls for all operating systems
    categorization.render_known_syscalls(&env, &oses, "README.md.j2", "README.md")?;

    // Render unknown system calls for each OS
    for os in oses {
        let dest_path = format!("UNKNOWN-{}.md", os.name().to_uppercase());

        categorization.render_unknown_syscalls(&env, os.as_ref(), "UNKNOWN.md.j2", &dest_path)?;
    }

    Ok(())
}
