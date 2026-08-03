// src/pkg.rs

use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;
use tar::{Archive, Builder};

const REGISTRY_URL: &str = "https://cpm.ultimatecraw.xyz/api";
const CPM_CONFIG_FILE: &str = "config.toml";

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct PackageToml {
    name: String,
    version: String,
    description: Option<String>,
    author: Option<String>,
    license: Option<String>,
    #[serde(default)]
    repository: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct CrawConfig {
    #[serde(default)]
    registry: RegistryConfig,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct RegistryConfig {
    url: Option<String>,
    api_key: Option<String>,
    username: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CurrentUserResponse {
    username: String,
    display_name: String,
    created_at: String,
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
    readme: Option<String>,
    url: String,
    sha256: String,
}

fn read_package_readme(root: &Path) -> Result<Option<String>, Box<dyn Error>> {
    const MAX_README_BYTES: u64 = 100_000;

    let candidates = ["README.md", "Readme.md", "readme.md"];

    for candidate in candidates {
        let path = root.join(candidate);

        if !path.exists() {
            continue;
        }

        let metadata = fs::metadata(&path)?;

        if metadata.len() > MAX_README_BYTES {
            return Err(format!("{} exceeds the 100 KB README limit.", path.display(),).into());
        }

        return Ok(Some(fs::read_to_string(path)?));
    }

    Ok(None)
}

fn publish_to_registry(
    registry: &str,
    api_key: &str,
    config: &CrawConfig,
    package: &PackageToml,
    readme: Option<String>,
    url: &str,
    sha256: &str,
) -> Result<(), Box<dyn Error>> {
    println!("Publishing {} v{} to CPM...", package.name, package.version,);

    let request = PublishPackageRequest {
        name: package.name.clone(),
        version: package.version.clone(),
        description: package.description.clone(),
        author: package.author.clone(),
        license: package.license.clone(),
        readme,
        url: url.to_string(),
        sha256: sha256.to_string(),
    };

    let endpoint = format!("{}/package", registry.trim_end_matches('/'),);

    let body = serde_json::to_string(&request)?;

    let response = ureq::post(&endpoint)
        .set("Authorization", &format!("Bearer {}", api_key))
        .set("Content-Type", "application/json")
        .send_string(&body);

    match response {
        Ok(response) if response.status() == 200 || response.status() == 201 => {
            let username = config.registry.username.as_deref().unwrap_or("unknown");

            println!(
                "Published {} v{} as @{}",
                package.name, package.version, username,
            );

            println!("URL: {}", url);
            println!("SHA-256: {}", sha256);

            Ok(())
        }

        Ok(response) => {
            Err(format!("Publish failed with HTTP status {}.", response.status(),).into())
        }

        Err(ureq::Error::Status(code, response)) => {
            let body = response.into_string().unwrap_or_default();

            let message = match code {
                400 => {
                    if body.trim().is_empty() {
                        "The registry rejected the package data.".to_string()
                    } else {
                        format!("The registry rejected the package:\n{}", body,)
                    }
                }

                401 => "Your CPM API key is invalid or revoked.\n\n\
           Run:\n  craw pkg login"
                    .to_string(),

                403 => {
                    if body.trim().is_empty() {
                        format!("You do not own the package '{}'.", package.name,)
                    } else {
                        body
                    }
                }

                409 => format!(
                    "{} v{} already exists on CPM.\n\n\
           Bump the version in package.toml.",
                    package.name, package.version,
                ),

                413 => "The package README is too large.".to_string(),

                500 => "The CPM registry had an internal error.".to_string(),

                _ => {
                    if body.trim().is_empty() {
                        format!("Publish failed with HTTP status {}.", code,)
                    } else {
                        format!("Publish failed with HTTP status {}:\n{}", code, body,)
                    }
                }
            };

            Err(message.into())
        }

        Err(error) => Err(format!(
            "Could not reach CPM registry.\n\n\
         Registry: {}\nError: {}",
            endpoint, error,
        )
        .into()),
    }
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
            let explicit_url = match args {
                [_] => None,

                [_, flag, url] if flag == "--url" => Some(url.as_str()),

                _ => {
                    return Err("Usage: craw pkg publish [--url <archive-url>]".into());
                }
            };

            publish_package(explicit_url)?;
        }

        "login" => {
            login()?;
        }

        "whoami" => {
            whoami()?;
        }

        "logout" => {
            logout()?;
        }

        _ => print_help(),
    }

    Ok(())
}

fn pack_package_from_current_dir() -> Result<(), Box<dyn Error>> {
    let root = std::env::current_dir()?;
    let package = load_current_package(&root)?;

    validate_package_name(&package.name)?;
    validate_package_version(&package.version)?;

    let archive_path = root
        .join("target")
        .join(format!("{}-{}.tar.gz", package.name, package.version,));

    pack_package(&root, &package, &archive_path)?;

    let hash = sha256_file(&archive_path)?;

    println!("Packed {} v{}", package.name, package.version,);
    println!("Created {}", archive_path.display());
    println!("SHA-256: {}", hash);

    Ok(())
}

fn git_origin_url(root: &Path) -> Result<Option<String>, Box<dyn Error>> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["remote", "get-url", "origin"])
        .output();

    let output = match output {
        Ok(output) if output.status.success() => output,
        Ok(_) => return Ok(None),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(None);
        }
        Err(error) => return Err(error.into()),
    };

    let url = String::from_utf8(output.stdout)?.trim().to_string();

    if url.is_empty() {
        Ok(None)
    } else {
        Ok(Some(url))
    }
}

fn github_repo_from_url(remote: &str) -> Option<String> {
    let trimmed = remote.trim().trim_end_matches('/').trim_end_matches(".git");

    if let Some(repo) = trimmed.strip_prefix("https://github.com/") {
        return Some(repo.to_string());
    }

    if let Some(repo) = trimmed.strip_prefix("git@github.com:") {
        return Some(repo.to_string());
    }

    if let Some(repo) = trimmed.strip_prefix("ssh://git@github.com/") {
        return Some(repo.to_string());
    }

    None
}

fn validate_github_repo(repo: &str) -> Result<(), Box<dyn Error>> {
    let mut parts = repo.split('/');

    let owner = parts.next();
    let name = parts.next();

    if owner.is_none() || name.is_none() || parts.next().is_some() {
        return Err(format!("Invalid GitHub repository: {}", repo,).into());
    }

    Ok(())
}

fn publish_github_release(
    root: &Path,
    package: &PackageToml,
    archive_path: &Path,
) -> Result<String, Box<dyn Error>> {
    ensure_gh_available()?;
    ensure_gh_authenticated()?;

    let remote = git_origin_url(root)?.ok_or(
        "No Git origin remote was found.\n\n\
       Add a GitHub remote or set repository in package.toml.",
    )?;

    let repo = github_repo_from_url(&remote).ok_or_else(|| {
        format!(
            "The origin remote is not a supported GitHub URL:\n  {}",
            remote,
        )
    })?;

    validate_github_repo(&repo)?;

    let tag = format!("v{}", package.version);
    let asset_name = archive_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("Invalid archive filename")?;

    println!("Creating GitHub release {}...", tag);

    let status = Command::new("gh")
        .arg("release")
        .arg("create")
        .arg(&tag)
        .arg(archive_path)
        .arg("--repo")
        .arg(&repo)
        .arg("--title")
        .arg(format!("{} v{}", package.name, package.version,))
        .arg("--notes")
        .arg("Published with CPM")
        .status()?;

    if !status.success() {
        return Err("GitHub release creation failed.".into());
    }

    Ok(format!(
        "https://github.com/{}/releases/download/{}/{}",
        repo, tag, asset_name,
    ))
}

fn ensure_gh_available() -> Result<(), Box<dyn Error>> {
    match Command::new("gh").arg("--version").output() {
        Ok(output) if output.status.success() => Ok(()),

        Ok(_) => Err("GitHub CLI is installed but not working correctly.".into()),

        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            Err("GitHub CLI is required for automatic publishing.\n\n\
         Install it, then run:\n  gh auth login\n\n\
         Alternatively:\n  craw pkg publish --url <archive-url>"
                .into())
        }

        Err(error) => Err(error.into()),
    }
}

fn ensure_gh_authenticated() -> Result<(), Box<dyn Error>> {
    let status = Command::new("gh")
        .args(["auth", "status", "--hostname", "github.com"])
        .status()?;

    if !status.success() {
        return Err("You are not signed into GitHub CLI.\n\nRun:\n  gh auth login".into());
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
        repository: None,
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
    println!("  gh auth login");
    println!("  craw pkg login");
    println!("  craw pkg publish");

    Ok(())
}

fn print_help() {
    println!("Crawssembly Package Manager (CPM)");
    println!();
    println!("Account commands:");
    println!("  craw pkg login");
    println!("    - signs into CPM using an API key");
    println!("  craw pkg logout");
    println!("    - removes the saved API key from this machine");
    println!("  craw pkg whoami");
    println!("    - shows the currently authenticated CPM account");
    println!();
    println!("Package commands:");
    println!("  craw pkg install <package|package.tar.gz>");
    println!("    - installs a package from CPM or a local archive");
    println!("  craw pkg list");
    println!("    - lists installed packages");
    println!("  craw pkg pack [package]");
    println!("    - creates an archive from the current or installed package");
    println!("  craw pkg publish");
    println!("    - packs and publishes the current package using GitHub Releases");
    println!("  craw pkg publish --url <archive-url>");
    println!("    - publishes using an externally hosted archive");
    println!("  craw pkg remove <package>");
    println!("    - removes an installed package");
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

fn config_path() -> Result<PathBuf, Box<dyn Error>> {
    Ok(craw_home()?.join(CPM_CONFIG_FILE))
}

fn load_config() -> Result<CrawConfig, Box<dyn Error>> {
    let path = config_path()?;

    if !path.exists() {
        return Ok(CrawConfig::default());
    }

    let text = fs::read_to_string(&path).map_err(|e| {
        format!(
            "Could not read CPM configuration at '{}': {}",
            path.display(),
            e
        )
    })?;

    let config = toml::from_str(&text).map_err(|e| {
        format!(
            "CPM configuration at '{}' is invalid:\n{}",
            path.display(),
            e
        )
    })?;

    Ok(config)
}

fn save_config(config: &CrawConfig) -> Result<(), Box<dyn Error>> {
    let home = craw_home()?;
    fs::create_dir_all(&home)?;

    let path = config_path()?;
    let text = toml::to_string_pretty(config)?;

    fs::write(&path, text)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        fs::set_permissions(&path, fs::Permissions::from_mode(0o600))?;
    }

    Ok(())
}

fn registry_url(config: &CrawConfig) -> &str {
    config.registry.url.as_deref().unwrap_or(REGISTRY_URL)
}

fn fetch_current_user(
    registry: &str,
    api_key: &str,
) -> Result<CurrentUserResponse, Box<dyn Error>> {
    let endpoint = format!("{}/me", registry.trim_end_matches('/'));

    let response = ureq::get(&endpoint)
        .set("Authorization", &format!("Bearer {}", api_key))
        .call();

    match response {
        Ok(response) => {
            let text = response.into_string()?;

            let user: CurrentUserResponse = serde_json::from_str(&text).map_err(|e| {
                format!(
                    "The CPM registry returned an invalid account response:\n{}",
                    e
                )
            })?;

            Ok(user)
        }

        Err(ureq::Error::Status(401, _)) => {
            Err("CPM rejected the API key.\n\nCheck the key and try again.".into())
        }

        Err(ureq::Error::Status(code, response)) => {
            let body = response.into_string().unwrap_or_default();

            if body.trim().is_empty() {
                Err(format!("CPM returned HTTP status {}.", code).into())
            } else {
                Err(format!("CPM returned HTTP status {}:\n{}", code, body).into())
            }
        }

        Err(error) => Err(format!(
            "Could not reach the CPM registry.\n\nRegistry: {}\nError: {}",
            endpoint, error
        )
        .into()),
    }
}

fn require_api_key(config: &CrawConfig) -> Result<&str, Box<dyn Error>> {
    config
        .registry
        .api_key
        .as_deref()
        .ok_or_else(|| "You are not signed into CPM.\n\nRun:\n  craw pkg login".into())
}

fn login() -> Result<(), Box<dyn Error>> {
    let mut config = load_config()?;
    let registry = registry_url(&config).to_string();

    println!("Sign into CPM");
    println!("Registry: {}", registry);
    println!();
    println!("Create or manage your account at:");
    println!("  https://cpm.ultimatecraw.xyz/account.html");
    println!();

    let api_key = rpassword::prompt_password("API key: ")?;
    let api_key = api_key.trim();

    if api_key.is_empty() {
        return Err("API key cannot be empty.".into());
    }

    if !api_key.starts_with("cpm_") {
        return Err("That does not look like a CPM API key.\nCPM keys begin with 'cpm_'.".into());
    }

    println!("Checking API key...");

    let user = fetch_current_user(&registry, api_key)?;

    config.registry.url = Some(registry);
    config.registry.api_key = Some(api_key.to_string());
    config.registry.username = Some(user.username.clone());

    save_config(&config)?;

    println!();
    println!(
        "Signed into CPM as {} (@{}).",
        user.display_name, user.username
    );

    println!("Credentials saved to:");
    println!("  {}", config_path()?.display());

    Ok(())
}

fn fetch_remote_package(name: &str) -> Result<RemotePackage, Box<dyn Error>> {
    let url = format!("{}/package/{}", REGISTRY_URL, name);

    let text = ureq::get(&url).call()?.into_string()?;

    let package: RemotePackage = serde_json::from_str(&text)?;

    Ok(package)
}

fn logout() -> Result<(), Box<dyn Error>> {
    let mut config = load_config()?;

    if config.registry.api_key.is_none() {
        println!("You are not signed into CPM.");
        return Ok(());
    }

    let username = config
        .registry
        .username
        .clone()
        .unwrap_or_else(|| "current user".to_string());

    config.registry.api_key = None;
    config.registry.username = None;

    save_config(&config)?;

    println!("Signed out of CPM account @{}.", username);

    Ok(())
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
            package.name, package.sha256, actual_hash
        )
        .into());
    }

    install_local_package(&archive_path)?;

    if let Err(error) = report_install(&package.name, &package.latest) {
        eprintln!(
            "Warning: package installed, but CPM statistics could not be updated: {}",
            error
        );
    }

    Ok(())
}

#[derive(Debug, Serialize)]
struct InstallReportRequest {
    version: String,
}

fn report_install(name: &str, version: &str) -> Result<(), Box<dyn Error>> {
    let endpoint = format!("{}/package/{}/install", REGISTRY_URL, name,);

    let body = serde_json::to_string(&InstallReportRequest {
        version: version.to_string(),
    })?;

    let response = ureq::post(&endpoint)
        .set("Content-Type", "application/json")
        .send_string(&body);

    match response {
        Ok(response) if response.status() == 200 || response.status() == 204 => Ok(()),

        Ok(response) => Err(format!("CPM returned HTTP status {}", response.status(),).into()),

        Err(ureq::Error::Status(code, response)) => {
            let body = response.into_string().unwrap_or_default();

            if body.trim().is_empty() {
                Err(format!("CPM returned HTTP status {}", code,).into())
            } else {
                Err(format!("CPM returned HTTP status {}: {}", code, body,).into())
            }
        }

        Err(error) => Err(format!("Could not reach CPM: {}", error,).into()),
    }
}

fn whoami() -> Result<(), Box<dyn Error>> {
    let config = load_config()?;
    let api_key = require_api_key(&config)?;
    let registry = registry_url(&config);

    let user = fetch_current_user(registry, api_key)?;

    println!("{} (@{})", user.display_name, user.username);
    println!("CPM member since {}", user.created_at);
    println!("Registry: {}", registry);

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

    db.packages.insert(
        package.name.clone(),
        InstalledPackage {
            version: package.version.clone(),
            files: installed_files,
        },
    );

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
            copy_package_files(&source_path, &target_path, package_name, installed_files)?;
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

fn load_current_package(root: &Path) -> Result<PackageToml, Box<dyn Error>> {
    let metadata_path = root.join("package.toml");

    if !metadata_path.exists() {
        return Err("No package.toml found.\n\n\
       Run this command from a package directory."
            .into());
    }

    let metadata_text = fs::read_to_string(&metadata_path)
        .map_err(|error| format!("Could not read '{}': {}", metadata_path.display(), error,))?;

    let package: PackageToml = toml::from_str(&metadata_text)
        .map_err(|error| format!("package.toml is invalid:\n{}", error,))?;

    Ok(package)
}

fn validate_publish_url(url: &str) -> Result<(), Box<dyn Error>> {
    if !url.starts_with("https://") {
        return Err("Publish URL must start with https://".into());
    }

    Ok(())
}

fn pack_package(
    root: &Path,
    package: &PackageToml,
    archive_path: &Path,
) -> Result<(), Box<dyn Error>> {
    let metadata_path = root.join("package.toml");

    let out_dir = archive_path.parent().ok_or("Invalid archive output path")?;

    fs::create_dir_all(out_dir)?;

    let file = File::create(archive_path)?;
    let encoder = GzEncoder::new(file, Compression::default());

    let mut tar = Builder::new(encoder);

    tar.append_path_with_name(&metadata_path, "package.toml")?;

    append_package_dir(&mut tar, root, &format!("std/{}", package.name))?;

    tar.finish()?;

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
        repository: None,
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

fn publish_package(explicit_url: Option<&str>) -> Result<(), Box<dyn Error>> {
    let root = std::env::current_dir()?;
    let package = load_current_package(&root)?;

    validate_package_name(&package.name)?;
    validate_package_version(&package.version)?;

    let config = load_config()?;
    let api_key = require_api_key(&config)?;
    let registry = registry_url(&config);

    let archive_path = root
        .join("target")
        .join(format!("{}-{}.tar.gz", package.name, package.version,));

    println!("Packing {} v{}...", package.name, package.version,);

    pack_package(&root, &package, &archive_path)?;

    let sha256 = sha256_file(&archive_path)?;
    let readme = read_package_readme(&root)?;

    let url = match explicit_url {
        Some(url) => {
            validate_publish_url(url)?;
            url.to_string()
        }

        None => publish_github_release(&root, &package, &archive_path)?,
    };

    publish_to_registry(registry, api_key, &config, &package, readme, &url, &sha256)
}

#[allow(dead_code)]
fn ensure_package_archive(root: &Path, package: &PackageToml) -> Result<PathBuf, Box<dyn Error>> {
    let archive_path = root
        .join("target")
        .join(format!("{}-{}.tar.gz", package.name, package.version,));

    println!("Packing {} v{}...", package.name, package.version,);

    pack_package(root, package, &archive_path)?;

    Ok(archive_path)
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
        && !path
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
}

fn validate_package_name(name: &str) -> Result<(), Box<dyn Error>> {
    if name.is_empty() {
        return Err("Package name cannot be empty.".into());
    }

    let valid = name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_');

    if !valid {
        return Err(
            "Package names can only contain lowercase letters, numbers, '-' and '_'.".into(),
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
        return Err("Package version can only contain letters, numbers, '.', '-' and '_'.".into());
    }

    Ok(())
}
