// src/pck.rs

use std::fs;
use std::fs::File;
use std::error::Error;
use std::path::{Path, PathBuf};

use tar::Archive;
use sha2::{Digest, Sha256};
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};

const REGISTRY_URL: &str = "https://cpm.ultimatecraw.xyz/api";

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct PackageToml {
    name: String,
    version: String,
    description: Option<String>,
    author: Option<String>,
    license: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct InstalledPackage {
    version: String,
    files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct InstalledDb {
    packages: std::collections::BTreeMap<String, InstalledPackage>,
}

#[derive(Debug, Deserialize)]
struct RemotePackage {
    name: String,
    latest: String,
    url: String,
    sha256: String,
}

fn fetch_remote_package(name: &str) -> Result<RemotePackage, Box<dyn Error>> {
    let url = format!("{}/package/{}", REGISTRY_URL, name);

    let text = ureq::get(&url)
        .call()?
        .into_string()?;

    let package: RemotePackage = serde_json::from_str(&text)?;

    Ok(package)
}

fn install_registry_package(name: &str) -> Result<(), Box<dyn Error>> {
    let package = fetch_remote_package(name)?;

    fs::create_dir_all(packages_dir()?.join("cache"))?;

    let archive_path = packages_dir()?
        .join("cache")
        .join(format!("{}-{}.tar.gz", package.name, package.latest));

    println!("Downloading {} v{}...", package.name, package.latest);

    download_file(&package.url, &archive_path)?;

    let actual_hash = sha256_file(&archive_path)?;

    if actual_hash != package.sha256 {
        return Err(format!(
            "Checksum mismatch for '{}'\nExpected: {}\nActual: {}",
            package.name,
            package.sha256,
            actual_hash
        ).into());
    }

    install_local_package(&archive_path)?;

    Ok(())
}

pub fn handle_pkg(args: &[String]) -> Result<(), Box<dyn Error>> {
    if args.is_empty() {
        print_help();
        return Ok(());
    }

    match args[0].as_str() {
        "install" => {
            if args.len() < 2 {
              return Err("Usage: craw pks install <package|package.tar.gz>".into());
            }

            let input = &args[1];

            if Path::new(input).exists() {
                install_local_package(Path::new(input))?;
            } else {
                install_registry_package(input)?;
            }
        }

        "list" => { list_packages()?; }

        "remove" => {
            if args.len() < 2 { return Err("Usage: craw pkg remove <name>".into()); }
            remove_package(&args[1])?;
        }

        _ => print_help(),
    }

    Ok(())
}

fn print_help() {
    println!("Crawssembly Package Manager (CPM)");
    println!();
    println!("Commands:");
    println!("  craw pkg install <package|package.tar.gz>");
    println!("  craw pkg list");
    println!("  craw pkg remove <package>");
}

fn craw_home() -> Result<PathBuf, Box<dyn Error>> {
    let home = dirs::home_dir().ok_or("Could not find home directory")?;
    Ok(home.join(".crawssembly"))
}

fn std_dir() -> Result<PathBuf, Box<dyn Error>> {
    Ok(craw_home()?.join("std"))
}

fn packages_dir() -> Result<PathBuf, Box<dyn Error>> {
    Ok(craw_home()?.join("packages"))
}

fn installed_db_path() -> Result<PathBuf, Box<dyn Error>> {
    Ok(packages_dir()?.join("installed.toml"))
}

fn is_safe_path(path: &Path) -> bool {
    !path.is_absolute()
        && !path.components().any(|c| {
            matches!(c, std::path::Component::ParentDir)
        })
}

fn install_local_package(path: &Path) -> Result<(), Box<dyn Error>> {
    let temp = std::env::temp_dir().join("craw_pkg_install");

    if temp.exists() { fs::remove_dir_all(&temp)?; }

    fs::create_dir_all(&temp)?;

    let file = File::open(path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let entry_path = entry.path()?.to_path_buf();

        if !is_safe_path(&entry_path) { return Err(format!("Unsafe package path: {:?}", entry_path).into()); }

        entry.unpack(temp.join(entry_path))?;

    }

    let metadata_path = temp.join("package.toml");
    let metadata_text = fs::read_to_string(metadata_path)?;
    let package: PackageToml = toml::from_str(&metadata_text)?;

    let source_std = temp.join("std").join(&package.name);
    if !source_std.exists() { return Err(format!("Package missing std/{} folder", package.name).into()); }

    let target_std = std_dir()?.join(&package.name);

    if target_std.exists() { return Err(format!("Package '{}' is already installed", package.name).into()); }

    let mut installed_files = Vec::new();
    copy_package_files(
        &source_std,
        &target_std,
        &package.name,
        &mut installed_files,
    )?;

    let mut db = load_installed_db()?;

    db.packages.insert(package.name.clone(), InstalledPackage {
        version: package.version.clone(),
        files: installed_files,
    });

    save_installed_db(&db)?;

    println!("Installed {} v{}", package.name, package.version);

    Ok(())
}

fn copy_package_files(
    source: &Path,
    target: &Path,
    package_name: &str,
    installed_files: &mut Vec<String>,
) -> Result<(), Box<dyn Error>> {

    fs::create_dir_all(target)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());

        if source_path.is_dir() {
            copy_package_files(
                &source_path,
                &target_path,
                package_name,
                installed_files,
            )?;
        } else {
            fs::copy(&source_path, &target_path)?;

            let rel = target_path
                .strip_prefix(craw_home()?)?
                .to_string_lossy()
                .replace("\\", "/");

            if !rel.starts_with(&format!("std/{}/", package_name)) {
                return Err("Package tried to install outside its namespace".into());
            }

            installed_files.push(rel);

        }
    }

    Ok(())
}

fn load_installed_db() -> Result<InstalledDb, Box<dyn Error>> {
    let path = installed_db_path()?;

    if !path.exists() { return Ok(InstalledDb::default()); }

    let text = fs::read_to_string(path)?;
    Ok(toml::from_str(&text)?)
}

fn save_installed_db(db: &InstalledDb) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(packages_dir()?)?;

    let text = toml::to_string_pretty(db)?;
    fs::write(installed_db_path()?, text)?;

    Ok(())
}

fn list_packages() -> Result<(), Box<dyn Error>> {
    let db = load_installed_db()?;

    if db.packages.is_empty() {
        println!("No packages installed.");
        return Ok(());
    }

    for (name, package) in db.packages { println!("{} v{}", name, package.version); }

    Ok(())
}

fn remove_package(name: &str) -> Result<(), Box<dyn Error>> {
    let mut db = load_installed_db()?;

    let package = match db.packages.remove(name) {
        Some(package) => package,
        None => return Err(format!("Package '{}' is not installed", name).into()),
    };

    for file in package.files {
        let path = craw_home()?.join(file);

        if path.exists() { fs::remove_file(path)?; }
    }

    let package_dir = std_dir()?.join(name);

    if package_dir.exists() { let _ = fs::remove_dir_all(package_dir); }

    save_installed_db(&db)?;

    println!("Removed {}", name);

    Ok(())
}

fn download_file(url: &str, target: &Path) -> Result<(), Box<dyn Error>> {
    let mut response = ureq::get(url).call()?.into_reader();
    let mut file = File::create(target)?;

    std::io::copy(&mut response, &mut file)?;

    Ok(())
}

fn sha256_file(path: &Path) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();

    std::io::copy(&mut file, &mut hasher)?;

    Ok(hex::encode(hasher.finalize()))
}






