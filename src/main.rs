use std::fs;
use std::path::Path;
use toml::Value;
use reqwest::blocking::get;
use semver::Version;

fn read_pyproject_toml(file_path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let value = content.parse::<Value>()?;
    Ok(value)
}

fn read_uv_script(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    Ok(content)
}

fn find_latest_version(dependency: &str) -> Result<Version, Box<dyn std::error::Error>> {
    let url = format!("https://crates.io/api/v1/crates/{}", dependency);
    let response = get(&url)?.json::<Value>()?;
    let version_str = response["crate"]["max_version"].as_str().ok_or("No version found")?;
    let version = Version::parse(version_str)?;
    Ok(version)
}

fn upgrade_dependencies(dependencies: &mut Value) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(deps) = dependencies.as_table_mut() {
        for (name, version) in deps.iter_mut() {
            let latest_version = find_latest_version(name)?;
            *version = Value::String(latest_version.to_string());
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pyproject_path = "pyproject.toml";
    let uv_script_path = "uv_script.uv";

    let mut pyproject = read_pyproject_toml(pyproject_path)?;
    let uv_script = read_uv_script(uv_script_path)?;

    upgrade_dependencies(&mut pyproject)?;

    println!("Updated pyproject.toml: {:?}", pyproject);
    println!("Read uv script: {:?}", uv_script);

    Ok(())
}
