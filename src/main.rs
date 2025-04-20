use clap::{Parser, Subcommand};
use solana_program_inspector::{analyze_program, analyze_binary};
use std::{path::PathBuf, fs};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Analyze {
        program_id: String,
        #[clap(short, long, default_value_t = String::from("https://api.mainnet-beta.solana.com"))]
        rpc_url: String,
        #[clap(short, long)]
        verbose: bool,
    },
    AnalyzeBinary {
        path: PathBuf,
        #[clap(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Analyze { program_id, rpc_url, verbose } => {
            let analysis = analyze_program(&program_id, &rpc_url)?;
            if verbose {
                println!("{:#?}", analysis);
            } else {
                println!("{}", serde_json::to_string_pretty(&analysis)?);
            }
        }
        Commands::AnalyzeBinary { path, output } => {
            let data = fs::read(path)?;
            let analysis = analyze_binary(&data)?;
            
            if let Some(out_path) = output {
                fs::write(out_path, serde_json::to_string_pretty(&analysis)?);
            } else {
                println!("{}", serde_json::to_string_pretty(&analysis)?);
            }
        }
    }
    Ok(())
}