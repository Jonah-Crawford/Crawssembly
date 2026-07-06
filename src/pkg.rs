// src/pkg.rs

use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tar::{Archive, Builder};

const REGISTRY_URL: &str = "https://cpm.ultimatecraw.xyz/api";

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Serialize)]
struct PublishPackageRequest {
  name: String,
  version: String,
  description: Option<String>,
  author: Option<String>,
  license: Option<String>,
  url: String,
  sha256: String,
}

pub fn handle_pkg(args: &[String]) -> Result<(), Box<dyn Error>> {
  if args.is_empty() {
    print_help();
    return Ok(());
  }

  match args[0].as_str() {
    "install" => {
      if args.len() < 2 {
        return Err("Usage: craw pkg install <package|package.tar.gz>".into());
      }

      let input = &args[1];

      if Path::new(input).exists() {
        install_local_package(Path::new(input))?;
      } else {
        install_registry_package(input)?;
      }
    }

    "list" => {
      list_packages()?;
    }

    "remove" => {
      if args.len() < 2 {
        return Err("Usage: craw pkg remove <name>".into());
      }

      remove_package(&args[1])?;
    }

    "pack" => {
      if args.len() >= 2 {
        pack_package_from_std(&args[1])?;
      } else {
        pack_package_from_current_dir()?;
      }
    }

    "publish" => {
      if args.len() < 3 || args[1] != "--url" {
        return Err(
          "Usage: craw pkg publish --url <github-release-tarball-url>".into()
        );
      }

      publish_package(&args[2])?;
    }

    _ => print_help(),
  }

  Ok(())
}

pub fn init_package(name: &str) -> Result<(), Box<dyn Error>> {
  validate_package_name(name)?;

  let root = PathBuf::from(name);

  if root.exists() {
    return Err(format!("Directory '{}' already exists", name).into());
  }

  fs::create_dir_all(&root)?;

  let package_toml = PackageToml {
    name: name.to_string(),
    version: "1.0.0".to_string(),
    description: Some(format!("{}, a Crawssembly package.", name)),
    author: None,
    license: Some("MIT".to_string()),
  };

  let toml_text = toml::to_string_pretty(&package_toml)?;
  fs::write(root.join("package.toml"), toml_text)?;

  let hello = r#"sav 72 ref
sav 101 ref
sav 108 ref
sav 108 ref
sav 111 ref
sav 32 ref
sav 87 ref
sav 111 ref
sav 114 ref
sav 108 ref
sav 100 ref
sav 33 ref"#;

  fs::write(root.join("main.craw"), hello)?;

  println!("Created package '{}'", name);
  println!();
  println!("Next steps:");
  println!("  cd {}", name);
  println!("  craw pkg pack");

  Ok(())
}

fn print_help() {
  println!("Crawssembly Package Manager (CPM)");
  println!();
  println!("Commands:");
  println!("  craw pkg install <package|package.tar.gz>");
  println!("    - installs a package from CPM or a local tarball path");
  println!("  craw pkg list");
  println!("    - lists all installed packages");
  println!("  craw pkg pack [package]");
  println!("    - creates a tarball from the current package or installed std package");
  println!("  craw pkg publish --url <github-release-tarball-url>");
  println!("    - publishes the current package to CPM");
  println!("  craw pkg remove <package>");
  println!("    - removes a package from your machine");
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

fn install_local_package(path: &Path) -> Result<(), Box<dyn Error>> {
  let temp = std::env::temp_dir().join("craw_pkg_install");

  if temp.exists() {
    fs::remove_dir_all(&temp)?;
  }

  fs::create_dir_all(&temp)?;

  let file = File::open(path)
    .map_err(|e| format!("Could not open package archive '{}': {}", path.display(), e))?;

  let decoder = GzDecoder::new(file);
  let mut archive = Archive::new(decoder);

  for entry in archive.entries()? {
    let mut entry = entry?;
    let entry_path = entry.path()?.to_path_buf();

    if !is_safe_path(&entry_path) {
      return Err(format!("Unsafe package path: {:?}", entry_path).into());
    }

    let target = temp.join(entry_path);

    if let Some(parent) = target.parent() {
      fs::create_dir_all(parent)?;
    }

    entry.unpack(target)?;
  }

  let metadata_path = temp.join("package.toml");

  if !metadata_path.exists() {
    return Err("Package archive is missing package.toml.".into());
  }

  let metadata_text = fs::read_to_string(&metadata_path)
    .map_err(|_| "Could not read package.toml from package archive.")?;

  let package: PackageToml = toml::from_str(&metadata_text)
    .map_err(|e| format!("package.toml inside archive is invalid:\n{}", e))?;

  validate_package_name(&package.name)?;
  validate_package_version(&package.version)?;

  let source_std = temp.join("std").join(&package.name);

  if !source_std.exists() {
    return Err(format!("Package missing std/{} folder.", package.name).into());
  }

  let target_std = std_dir()?.join(&package.name);

  if target_std.exists() {
    return Err(format!("Package '{}' is already installed.", package.name).into());
  }

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
        return Err("Package tried to install outside its namespace.".into());
      }

      installed_files.push(rel);
    }
  }

  Ok(())
}

fn pack_package_from_current_dir() -> Result<(), Box<dyn Error>> {
  let root = std::env::current_dir()?;
  let metadata_path = root.join("package.toml");

  if !metadata_path.exists() {
    return Err(
      "No package.toml found.\n\nCreate a package first:\n  craw init <name>"
        .into()
    );
  }

  let metadata_text = fs::read_to_string(&metadata_path)
    .map_err(|_| "Could not read package.toml.")?;

  let package: PackageToml = toml::from_str(&metadata_text)
    .map_err(|e| format!("package.toml is invalid:\n{}", e))?;

  validate_package_name(&package.name)?;
  validate_package_version(&package.version)?;

  let out_dir = root.join("target");
  fs::create_dir_all(&out_dir)?;

  let archive_path = out_dir.join(format!(
    "{}-{}.tar.gz",
    package.name,
    package.version
  ));

  let file = File::create(&archive_path)?;
  let encoder = GzEncoder::new(file, Compression::default());
  let mut tar = Builder::new(encoder);

  tar.append_path_with_name(&metadata_path, "package.toml")?;

  append_package_dir(
    &mut tar,
    &root,
    &format!("std/{}", package.name),
  )?;

  tar.finish()?;

  let hash = sha256_file(&archive_path)?;

  println!("Packed {} v{}", package.name, package.version);
  println!("Created {}", archive_path.display());
  println!("SHA-256: {}", hash);

  Ok(())
}

fn pack_package_from_std(name: &str) -> Result<(), Box<dyn Error>> {
  validate_package_name(name)?;

  let source_std = std_dir()?.join(name);

  if !source_std.exists() {
    return Err(format!("Package '{}' does not exist in std.", name).into());
  }

  let db = load_installed_db()?;

  let version = match db.packages.get(name) {
    Some(package) => package.version.clone(),
    None => "1.0.0".to_string(),
  };

  validate_package_version(&version)?;

  let package_toml = PackageToml {
    name: name.to_string(),
    version: version.clone(),
    description: None,
    author: None,
    license: None,
  };

  let out_dir = packages_dir()?.join("packed");
  fs::create_dir_all(&out_dir)?;

  let archive_path = out_dir.join(format!("{}-{}.tar.gz", name, version));

  let file = File::create(&archive_path)?;
  let encoder = GzEncoder::new(file, Compression::default());
  let mut tar = Builder::new(encoder);

  let metadata = toml::to_string_pretty(&package_toml)?;
  let mut header = tar::Header::new_gnu();

  header.set_path("package.toml")?;
  header.set_size(metadata.len() as u64);
  header.set_mode(0o644);
  header.set_cksum();

  tar.append(&header, metadata.as_bytes())?;
  tar.append_dir_all(format!("std/{}", name), &source_std)?;
  tar.finish()?;

  let hash = sha256_file(&archive_path)?;

  println!("Packed {} v{}", name, version);
  println!("Created {}", archive_path.display());
  println!("SHA-256: {}", hash);

  Ok(())
}

fn append_package_dir(
  tar: &mut Builder<GzEncoder<File>>,
  source: &Path,
  archive_prefix: &str,
) -> Result<(), Box<dyn Error>> {
  for entry in fs::read_dir(source)? {
    let entry = entry?;
    let path = entry.path();
    let name = entry.file_name();
    let name_str = name.to_string_lossy();

    if name_str == "package.toml" || name_str == "target" {
      continue;
    }

    let archive_path = format!("{}/{}", archive_prefix, name_str);

    if path.is_dir() {
      tar.append_dir(&archive_path, &path)?;
      append_package_dir(tar, &path, &archive_path)?;
    } else {
      tar.append_path_with_name(&path, &archive_path)?;
    }
  }

  Ok(())
}

fn publish_package(url: &str) -> Result<(), Box<dyn Error>> {
  let root = std::env::current_dir()?;
  let metadata_path = root.join("package.toml");

  if !metadata_path.exists() {
    return Err(
      "No package.toml found.\n\nRun this command from a package directory:\n  cd <package>\n  craw pkg publish --url <github-release-tarball-url>"
        .into()
    );
  }

  let metadata_text = fs::read_to_string(&metadata_path)
    .map_err(|_| "Could not read package.toml.")?;

  let package: PackageToml = toml::from_str(&metadata_text)
    .map_err(|e| format!("package.toml is invalid:\n{}", e))?;

  validate_package_name(&package.name)?;
  validate_package_version(&package.version)?;

  if !url.starts_with("https://") {
    return Err("Publish URL must start with https://".into());
  }

  let archive_path = root
    .join("target")
    .join(format!("{}-{}.tar.gz", package.name, package.version));

  if !archive_path.exists() {
    return Err(format!(
      "Package archive not found:\n  {}\n\nRun this first:\n  craw pkg pack",
      archive_path.display()
    ).into());
  }

  let token = std::env::var("CPM_PUBLISH_TOKEN").map_err(|_| {
    "Missing CPM_PUBLISH_TOKEN.\n\nSet it with:\n  export CPM_PUBLISH_TOKEN=\"your-token\""
  })?;

  println!("Publishing {} v{}...", package.name, package.version);

  let sha256 = sha256_file(&archive_path)?;

  let request = PublishPackageRequest {
    name: package.name.clone(),
    version: package.version.clone(),
    description: package.description.clone(),
    author: package.author.clone(),
    license: package.license.clone(),
    url: url.to_string(),
    sha256: sha256.clone(),
  };

  let endpoint = format!("{}/package", REGISTRY_URL);

  let body = serde_json::to_string(&request)?;

  let response = ureq::post(&endpoint)
    .set("Authorization", &format!("Bearer {}", token))
    .set("Content-Type", "application/json")
    .send_string(&body);

  match response {
    Ok(res) => {
      if res.status() == 200 || res.status() == 201 {
        println!("Published {} v{}", package.name, package.version);
        println!("URL: {}", url);
        println!("SHA-256: {}", sha256);
        Ok(())
      } else {
        Err(format!("Publish failed with HTTP status {}.", res.status()).into())
      }
    }

    Err(ureq::Error::Status(code, res)) => {
      let body = res.into_string().unwrap_or_default();

      let message = match code {
        400 => {
          if body.trim().is_empty() {
            "The registry rejected the package data.".to_string()
          } else {
            format!("The registry rejected the package:\n{}", body)
          }
        }

        401 => {
          "Invalid publish token.\n\nCheck CPM_PUBLISH_TOKEN and try again.".to_string()
        }

        404 => {
          format!(
            "The CPM registry does not have a publish endpoint at:\n  {}\n\nMake sure the server has POST /api/package implemented.",
            endpoint
          )
        }

        409 => {
          format!(
            "{} v{} already exists on CPM.\n\nBump the version in package.toml before publishing again.",
            package.name,
            package.version
          )
        }

        500 => {
          "The CPM registry had an internal error. Check the registry server logs.".to_string()
        }

        _ => {
          if body.trim().is_empty() {
            format!("Publish failed with HTTP status {}.", code)
          } else {
            format!("Publish failed with HTTP status {}:\n{}", code, body)
          }
        }
      };

      Err(message.into())
    }

    Err(e) => Err(format!(
      "Could not reach CPM registry.\n\nRegistry: {}\nError: {}",
      endpoint,
      e
    ).into()),
  }
}

fn load_installed_db() -> Result<InstalledDb, Box<dyn Error>> {
  let path = installed_db_path()?;

  if !path.exists() {
    return Ok(InstalledDb::default());
  }

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

  for (name, package) in db.packages {
    println!("{} v{}", name, package.version);
  }

  Ok(())
}

fn remove_package(name: &str) -> Result<(), Box<dyn Error>> {
  let mut db = load_installed_db()?;

  let package = match db.packages.remove(name) {
    Some(package) => package,
    None => return Err(format!("Package '{}' is not installed.", name).into()),
  };

  for file in package.files {
    let path = craw_home()?.join(file);

    if path.exists() {
      fs::remove_file(path)?;
    }
  }

  let package_dir = std_dir()?.join(name);

  if package_dir.exists() {
    let _ = fs::remove_dir_all(package_dir);
  }

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

fn is_safe_path(path: &Path) -> bool {
  !path.is_absolute()
    && !path.components().any(|c| {
      matches!(c, std::path::Component::ParentDir)
    })
}

fn validate_package_name(name: &str) -> Result<(), Box<dyn Error>> {
  if name.is_empty() {
    return Err("Package name cannot be empty.".into());
  }

  let valid = name.chars().all(|c| {
    c.is_ascii_lowercase()
      || c.is_ascii_digit()
      || c == '-'
      || c == '_'
  });

  if !valid {
    return Err(
      "Package names can only contain lowercase letters, numbers, '-' and '_'."
        .into()
    );
  }

  Ok(())
}

fn validate_package_version(version: &str) -> Result<(), Box<dyn Error>> {
  if version.trim().is_empty() {
    return Err("Package version cannot be empty.".into());
  }

  let valid = version.chars().all(|c| {
    c.is_ascii_digit()
      || c == '.'
      || c == '-'
      || c == '_'
      || c.is_ascii_lowercase()
      || c.is_ascii_uppercase()
  });

  if !valid {
    return Err(
      "Package version can only contain letters, numbers, '.', '-' and '_'."
        .into()
    );
  }

  Ok(())
}
