use clap::{Arg, Command};
use sysinfo::{DiskExt, System, SystemExt};
use tokio::fs as async_fs;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let matches = Command::new("Disk Usage Manager")
        .version("1.0")
        .about("View disk space usage and delete unwanted data")
        .arg(
            Arg::new("view")
                .short('v')
                .long("view")
                .help("View disk space usage")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("delete")
                .short('d')
                .long("delete")
                .num_args(1)
                .help("Delete a specified file or directory"),
        )
        .get_matches();

    if matches.get_flag("view") {
        view_disk_usage();
    }

    if let Some(path) = matches.get_one::<String>("delete") {
        delete_data(path).await;
    }
}

fn view_disk_usage() {
    let system = System::new_all();
    println!("Disk Usage:");
    for disk in system.disks() {
        println!(
            "Name: {:?}, Total: {} MB, Available: {} MB",
            disk.name(),
            disk.total_space() / 1_000_000,
            disk.available_space() / 1_000_000
        );
    }
}

async fn delete_data(path: &str) {
    let path = PathBuf::from(path);

    // Ensure path exists before trying to resolve it
    if !path.exists() {
        eprintln!("Error: Path {:?} does not exist.", path);
        return;
    }

    let abs_path = match path.canonicalize() {
        Ok(p) => p,
        Err(_) => {
            eprintln!("Error: Unable to resolve absolute path for {:?}", path);
            return;
        }
    };

    if abs_path.is_dir() {
        match async_fs::remove_dir_all(&abs_path).await {
            Ok(_) => println!("✅ Directory {:?} deleted successfully.", abs_path),
            Err(e) => eprintln!("❌ Failed to delete directory: {}", e),
        }
    } else if abs_path.is_file() {
        match async_fs::remove_file(&abs_path).await {
            Ok(_) => println!("✅ File {:?} deleted successfully.", abs_path),
            Err(e) => eprintln!("❌ Failed to delete file: {}", e),
        }
    } else {
        eprintln!("⚠️ Error: Path {:?} is neither a file nor a directory.", abs_path);
    }
}
