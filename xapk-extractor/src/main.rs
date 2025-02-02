use std::process::Command;
use std::path::Path;
use std::fs;
use std::io;
use zip::ZipArchive;
use std::env;
use thiserror::Error;
use anyhow::{Context, Result};

// Custom Error Type
#[derive(Error, Debug)]
pub enum XapkExtractorError {
    // Argument was empty
    #[error("Empty argument")]
    EmptyHost,

    // Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    // Represents all other cases of `anyhow::Error`.
    #[error(transparent)]
    AnyHowError(#[from] anyhow::Error),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Get XAPK path from command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <xapk_file>", args[0]);
        return Err(XapkExtractorError::EmptyHost.into());
    }

    let xapk_path = &args[1];

    // 2. Extract XAPK files
    let output_dir = format!("extracted_{}", Path::new(xapk_path).file_stem().unwrap_or_default().to_string_lossy()); // Dynamic output dir

    // 3. Create output directory
    fs::create_dir_all(&output_dir).context("Failed to create output directory")?;

    // 4. Open the XAPK file
    let file = fs::File::open(xapk_path).context("Failed to open XAPK file")?;
    let mut archive = ZipArchive::new(file).context("Failed to read XAPK file as ZIP archive")?;

    // 5. Extract files from the archive
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).context("Failed to access file in ZIP archive")?;
        let outpath = format!("{}/{}", output_dir, file.name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath).context("Failed to create directory in output path")?;
        } else {
            if let Some(p) = Path::new(&outpath).parent() {
                if !p.exists() {
                    fs::create_dir_all(p).context("Failed to create parent directory in output path")?;
                }
            }
            let mut outfile = fs::File::create(&outpath).context("Failed to create file in output path")?;
            io::copy(&mut file, &mut outfile).context("Failed to copy file contents to output path")?;
        }
    }

    println!("XAPK extraction completed successfully!");

    // 3. Install APKs using adb install-multiple
    let apks_dir = output_dir.clone();
    let apks = fs::read_dir(&apks_dir)?;

    let mut apk_files: Vec<String> = Vec::new();
    for entry in apks {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "apk") {
            apk_files.push(path.to_str().unwrap().to_string());
        } else {
            println!("Skipping non-APK file: {}", path.to_str().unwrap());
        }
    }

    if apk_files.is_empty() {
        println!("No APK files found in {}", apks_dir);
        return Ok(()); // Or return an error if you require APKs
    }

    let mut adb_command = Command::new("adb");
    adb_command.arg("install-multiple");

    for apk in &apk_files {
        adb_command.arg(apk);
    }

    let output = adb_command.output()?;

    if output.status.success() {
        println!("APK installation successful!");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("APK installation failed!");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        return Err("ADB installation failed".into());
    }

    Ok(())
}