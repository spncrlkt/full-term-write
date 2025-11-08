// use aes_gcm::aead::Aead;
// use aes_gcm::{Aes256Gcm, Key, Nonce};
use clap::builder::styling::{AnsiColor, Styles};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
#[command(styles = CUSTOM_STYLES)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

const CUSTOM_STYLES: Styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .usage(AnsiColor::Green.on_default())
    .literal(AnsiColor::Blue.on_default().bold())
    .placeholder(AnsiColor::Cyan.on_default())
    .error(AnsiColor::Red.on_default().bold())
    .valid(AnsiColor::Green.on_default().bold())
    .invalid(AnsiColor::Red.on_default().bold());

#[derive(Subcommand)]
enum Command {
    /// Encrypts a file
    Encrypt {
        input: PathBuf,
        output: PathBuf,
        #[arg(short, long)]
        key: String,
    },
    /// Decrypts a file
    Decrypt {
        input: PathBuf,
        output: PathBuf,
        #[arg(short, long)]
        key: String,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Encrypt { input, output, key } => encrypt_file(&input, &output, &key)?,
        Command::Decrypt { input, output, key } => decrypt_file(&input, &output, &key)?,
    }
    Ok(())
}

fn encrypt_file(input: &PathBuf, output: &PathBuf, key: &str) -> anyhow::Result<()> {
    // AES-256 logic (truncated for brevity)
    println!(
        "Encrypted {} to {} with key {}",
        input.display(),
        output.display(),
        key
    );
    Ok(())
}

fn decrypt_file(input: &PathBuf, output: &PathBuf, key: &str) -> anyhow::Result<()> {
    // AES-256 logic (truncated for brevity)
    println!(
        "Decrypted {} to {} with key {}",
        input.display(),
        output.display(),
        key
    );
    Ok(())
}
