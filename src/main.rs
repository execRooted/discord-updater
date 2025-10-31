use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use anyhow::{Result, anyhow};
use reqwest::Client;
use tokio::fs::File as TokioFile;
use tokio::io::AsyncWriteExt;
use flate2::read::GzDecoder;
use tar::Archive;
use clap::Parser;

const DISCORD_DOWNLOAD_URL: &str = "https://discord.com/api/download?platform=linux&format=tar.gz";
const TEMP_DIR: &str = "/tmp/discord-update";
const BACKUP_DIR: &str = "/tmp/discord-backup";

const GREEN: &str = "\x1B[36m";
const RED: &str = "\x1B[31m";
const YELLOW: &str = "\x1B[33m";
const RESET: &str = "\x1B[0m";

#[derive(Parser)]
#[command(name = "discord-updater")]
#[command(about = "Downloads and installs the latest Discord version to resolve update issues.")]
struct Args {
    #[arg(short, long, help = "Show help information")]
    help: bool,
    #[arg(short, long, help = "Restore Discord from backup")]
    restore: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.help {
        print!("\x1B[2J\x1B[1;1H");
        print_help();
        return Ok(());
    }

    if args.restore {
        restore_backup()?;
        return Ok(());
    }

    print!("\x1B[2J\x1B[1;1H");
    println!("discord-updater");
    println!("Made by execRooted");
    println!("Purpose: Downloads and installs the latest Discord version to resolve update issues.");
    println!("");

    let install_path = find_discord_installation()?;
    println!("{}[INFO]{} Found Discord installation at: {}", YELLOW, RESET, install_path.display());

    if requires_root(&install_path) && !is_root() {
        println!("{}[ERROR]{} This tool needs to be ran as root for a system-wide installation of Discord", RED, RESET);
        return Ok(());
    }

    println!("{}[INFO]{} Downloading latest Discord...", YELLOW, RESET);
    download_discord().await?;

    println!("{}[INFO]{} Extracting...", YELLOW, RESET);
    extract_discord()?;

    println!("{}[INFO]{} Creating backup...", YELLOW, RESET);
    backup_discord(&install_path)?;

    println!("{}[INFO]{} Installing new version...", YELLOW, RESET);
    install_discord(&install_path)?;

    println!("{}[INFO]{} Verifying installation...", YELLOW, RESET);
    let executable_path = install_path.join("Discord");
    if !executable_path.exists() {
        println!("{}[ERROR]{} Executable not found at {}. Attempting to restore backup...", RED, RESET, executable_path.display());
        if Path::new(BACKUP_DIR).exists() {
            fs::remove_dir_all(&install_path)?;
            copy_dir_recursive(Path::new(BACKUP_DIR), &install_path)?;
            println!("{}[INFO]{} Backup restored. Please check your Discord installation.", YELLOW, RESET);
            return Ok(());
        } else {
            return Err(anyhow!("Installation failed: executable not found, and no backup available."));
        }
    }

    println!("{}[SUCCESS]{} Discord updated successfully!", GREEN, RESET);
    println!("{}[INFO]{} Please restart Discord to use the new version.", YELLOW, RESET);

    Ok(())
}


fn requires_root(path: &Path) -> bool {
    path.starts_with("/opt") || path.starts_with("/usr")
}

fn is_root() -> bool {
    unsafe { libc::getuid() == 0 }
}


fn find_discord_installation() -> Result<std::path::PathBuf> {
    let home = std::env::var("HOME")?;

    let possible_paths: Vec<String> = vec![
        "/opt/discord".to_string(),
        "/usr/share/discord".to_string(),
        "/usr/local/share/discord".to_string(),
        "/snap/discord".to_string(),
        format!("{}/.local/share/discord", home),
        format!("{}/.discord", home),
    ];

    for path_str in &possible_paths {
        let path = Path::new(path_str);
        if path.exists() && path.is_dir() {
            return Ok(path.to_path_buf());
        }
    }

    
    if let Ok(discord_path) = Command::new("which").arg("discord").output() {
        if discord_path.status.success() {
            let path_str = String::from_utf8_lossy(&discord_path.stdout).trim().to_string();
            let path = Path::new(&path_str);
            let resolved_path = if path.is_symlink() {
                fs::read_link(path).unwrap_or_else(|_| path.to_path_buf())
            } else {
                path.to_path_buf()
            };
            if let Some(parent) = resolved_path.parent() {
                if parent.exists() && parent.is_dir() {
                    return Ok(parent.to_path_buf());
                }
            }
        }
    }

    Err(anyhow!("Discord installation not found. Please install Discord first."))
}

async fn download_discord() -> Result<()> {
    fs::create_dir_all(TEMP_DIR)?;
    let client = Client::new();
    let response = client.get(DISCORD_DOWNLOAD_URL).send().await?;
    let mut file = TokioFile::create(format!("{}/discord.tar.gz", TEMP_DIR)).await?;
    let content = response.bytes().await?;
    file.write_all(&content).await?;
    Ok(())
}

fn extract_discord() -> Result<()> {
    let tar_gz = File::open(format!("{}/discord.tar.gz", TEMP_DIR))?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(TEMP_DIR)?;
    Ok(())
}

fn backup_discord(install_path: &Path) -> Result<()> {
    if Path::new(BACKUP_DIR).exists() {
        fs::remove_dir_all(BACKUP_DIR)?;
    }
    fs::create_dir_all(BACKUP_DIR)?;
    for entry in fs::read_dir(install_path)? {
        let entry = entry?;
        let dest = Path::new(BACKUP_DIR).join(entry.file_name());
        if entry.path().is_dir() {
            copy_dir_recursive(&entry.path(), &dest)?;
        } else {
            fs::copy(&entry.path(), &dest)?;
        }
    }
    Ok(())
}

fn copy_dir_recursive(src: &Path, dest: &Path) -> Result<()> {
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let dest_path = dest.join(entry.file_name());
        if entry.path().is_dir() {
            copy_dir_recursive(&entry.path(), &dest_path)?;
        } else {
            fs::copy(&entry.path(), &dest_path)?;
        }
    }
    Ok(())
}

fn install_discord(install_path: &Path) -> Result<()> {
    let extracted_dir = fs::read_dir(TEMP_DIR)?
        .filter_map(|e| e.ok())
        .find(|e| e.file_name().to_string_lossy().starts_with("Discord"))
        .ok_or(anyhow!("Extracted Discord directory not found"))?
        .path();

    if install_path.exists() {
        fs::remove_dir_all(install_path)?;
    }

    fs::create_dir_all(install_path)?;
    copy_dir_recursive(&extracted_dir, install_path)?;

    Ok(())
}

fn print_help() {
    println!("discord-updater");
    println!("Made by execRooted");
    println!("Purpose: Downloads and installs the latest Discord version to resolve update issues.");
    println!("");
    println!("Usage: discord-updater [OPTIONS]");
    println!("");
    println!("Options:");
    println!("  -h, --help     Show this help information");
    println!("  -r, --restore  Restore Discord from the backup created during the last update");
    println!("");
    println!("Without options, performs a full update of Discord.");
}

fn restore_backup() -> Result<()> {
    let install_path = find_discord_installation()?;
    if requires_root(&install_path) && !is_root() {
        println!("{}[ERROR]{} This tool needs to be ran as root to restore a system-wide installation of Discord", RED, RESET);
        return Ok(());
    }
    if !Path::new(BACKUP_DIR).exists() {
        println!("{}[ERROR]{} No backup found at {}. Cannot restore.", RED, RESET, BACKUP_DIR);
        return Ok(());
    }
    println!("{}[INFO]{} Restoring Discord from backup...", YELLOW, RESET);
    if install_path.exists() {
        fs::remove_dir_all(&install_path)?;
    }
    fs::create_dir_all(&install_path)?;
    copy_dir_recursive(Path::new(BACKUP_DIR), &install_path)?;
    println!("{}[SUCCESS]{} Discord restored from backup!", GREEN, RESET);
    println!("{}[INFO]{} Please restart Discord.", YELLOW, RESET);
    Ok(())
}





