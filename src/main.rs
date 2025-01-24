use std::fs;
use toml::Value;
use reqwest::blocking::get;
use semver::Version;
use serde_json;

fn read_pyproject_toml(file_path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let value = content.parse::<Value>()?;
    Ok(value)
}

fn extract_package_name(dep_str: &str) -> &str {
    dep_str.split(&['=', '>', '<', '~', '!'][..])
        .next()
        .unwrap_or(dep_str)
        .trim()
}

fn find_latest_version(dependency: &str) -> Result<Version, Box<dyn std::error::Error>> {
    let package_name = extract_package_name(dependency);
    let url = format!("https://pypi.python.org/pypi/{}/json", package_name);
    let response = get(&url)?.json::<serde_json::Value>()?;

    // Get the releases object from the JSON response
    let releases = response.get("releases")
        .ok_or_else(|| "No releases found in PyPI response")?;

    // Find the latest version by comparing all release versions
    let latest_version = releases.as_object()
        .ok_or_else(|| "Releases is not an object")?
        .keys()
        .filter_map(|v| Version::parse(v).ok())
        .max()
        .ok_or_else(|| "No valid versions found")?;

    Ok(latest_version)
}

fn upgrade_dependencies(project: &mut Value) -> Result<(), Box<dyn std::error::Error>> {
    let dependencies = project.get_mut("project")
        .and_then(|p| p.get_mut("dependencies"))
        .ok_or("No project.dependencies found")?;

    match dependencies {
        Value::Array(deps) => {
            for dep in deps.iter_mut() {
                if let Some(dep_str) = dep.as_str() {
                    let package_name = extract_package_name(dep_str);
                    println!("Upgrading dependency: {}", package_name);
                    let latest_version = find_latest_version(dep_str)?;
                    *dep = Value::String(format!("{}=={}", package_name, latest_version));
                }
            }
        },
        Value::Table(deps) => {
            for (name, version) in deps.iter_mut() {
                println!("Upgrading dependency: {}", name);
                let latest_version = find_latest_version(name)?;
                *version = Value::String(format!("=={}", latest_version));
            }
        },
        _ => return Err("Dependencies must be an array or table".into()),
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pyproject_path = "pyproject.toml";
    let mut pyproject = read_pyproject_toml(pyproject_path)?;

    upgrade_dependencies(&mut pyproject)?;

    // Write back to file
    fs::write(pyproject_path, toml::to_string(&pyproject)?)?;
    println!("Updated pyproject.toml successfully");

    Ok(())
}
