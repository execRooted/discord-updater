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

const DISCORD_DOWNLOAD_URL: &str = "https://discord.com/api/download?platform=linux&format=tar.gz";
const TEMP_DIR: &str = "/tmp/discord-update";
const BACKUP_DIR: &str = "/tmp/discord-backup";

#[tokio::main]
async fn main() -> Result<()> {
    print!("\x1B[2J\x1B[1;1H");
    println!("discord-updater by execRooted");
    println!("Discord Updater for Linux");
    println!("This tool downloads the latest Discord version to fix the 'lucky day' issue.");
    println!("WARNING: This may require root privileges for system-wide installation.");

    let install_path = find_discord_installation()?;
    println!("Found Discord installation at: {}", install_path.display());

    if requires_root(&install_path) && !is_root() {
        println!("This operation requires root privileges.");
        println!("Please run with: sudo discord-updater");
        return Ok(());
    }

    println!("Downloading latest Discord...");
    download_discord().await?;

    println!("Extracting...");
    extract_discord()?;

    println!("Backing up current installation...");
    backup_discord(&install_path)?;

    println!("Installing new version...");
    install_discord(&install_path)?;

    println!("Discord updated successfully!");
    println!("Please restart Discord to use the new version.");

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

    let possible_paths = vec![
        "/opt/discord".to_string(),
        "/usr/share/discord".to_string(),
        "/usr/local/share/discord".to_string(),
        "/usr/bin/discord".to_string(),
        "/usr/local/bin/discord".to_string(),
        "/snap/discord".to_string(),
        format!("{}/.local/share/discord", home),
        format!("{}/.discord", home),
        "/var/lib/flatpak/exports/share/applications/com.discordapp.Discord.desktop".to_string(),
    ];

    for path_str in possible_paths {
        let path = Path::new(&path_str);
        if path.exists() {
            if path.is_dir() {
                return Ok(path.to_path_buf());
            } else if path.is_file() && path_str.contains("discord") {
                if let Some(parent) = path.parent() {
                    if parent.exists() {
                        return Ok(parent.to_path_buf());
                    }
                }
            }
        }
    }

    if let Ok(discord_path) = Command::new("which")
        .arg("discord")
        .output() {
        if discord_path.status.success() {
            let path_str = String::from_utf8_lossy(&discord_path.stdout).trim().to_string();
            let path = Path::new(&path_str);
            if let Some(parent) = path.parent() {
                if parent.exists() {
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
