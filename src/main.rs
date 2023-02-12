mod chunk;
mod chunk_type;
mod png;

use chunk::Chunk;
use chunk_type::ChunkType;
use clap::{Parser, Subcommand};
use png::Png;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Encode {
        file_path: PathBuf,
        chunk_type: ChunkType,
        message: String,
        output_path: PathBuf,
    },
    Decode {
        file_path: PathBuf,
        chunk_type: ChunkType,
    },
    Remove {
        file_path: PathBuf,
        chunk_type: ChunkType,
    },
    Print {
        file_path: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode {
            file_path,
            chunk_type,
            message,
            output_path,
        } => {
            let content = std::fs::read(file_path)?;
            let mut png = Png::try_from(content.as_ref())?;

            let message = message.into_bytes().to_vec();
            let chunk = Chunk::new(chunk_type, message);

            png.append_chunk(chunk);

            std::fs::write(output_path, png.as_bytes())?;
        }
        Commands::Decode {
            file_path,
            chunk_type,
        } => {
            let content = std::fs::read(file_path)?;
            let png = Png::try_from(content.as_ref())?;

            let chunk = png.chunk_by_type(&chunk_type.to_string());
            let chunk = chunk.ok_or("chunk not found")?.data_as_string()?;
            println!("{}", chunk);
        }
        Commands::Remove {
            file_path,
            chunk_type,
        } => {
            let content = std::fs::read(&file_path)?;
            let mut png = Png::try_from(content.as_ref())?;

            png.remove_chunk(&chunk_type.to_string());

            std::fs::write(file_path, png.as_bytes())?;
        }
        Commands::Print { file_path } => {
            let content = std::fs::read(file_path)?;
            let png = Png::try_from(content.as_ref())?;

            println!("{}", png);
        }
    }

    Ok(())
}
