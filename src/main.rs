use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about = "Inspects file metadata")]
struct Args {
    /// File path to inspect
    path: PathBuf,

    /// Show human-readable sizes (e.g., KB, MB)
    #[arg(short = 'H', long)]
    human_readable: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let metadata = std::fs::metadata(&args.path)?;

    let size = if args.human_readable {
        format!("{} KB", metadata.len() / 1024)
    } else {
        format!("{} bytes", metadata.len())
    };

    println!("Path: {:?}\nSize: {}", args.path, size);
    Ok(())
}
