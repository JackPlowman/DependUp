use std::fs;
use toml::Value;
use reqwest::blocking::get;
use semver::Version;
use depup::{read_pyproject_toml, read_uv_script, find_latest_version, upgrade_dependencies};

#[test]
fn test_read_pyproject_toml() {
    let pyproject_path = "tests/test_pyproject.toml";
    let pyproject = read_pyproject_toml(pyproject_path).expect("Failed to read pyproject.toml");
    assert!(pyproject.is_table());
}

#[test]
fn test_read_uv_script() {
    let uv_script_path = "tests/test_uv_script.uv";
    let uv_script = read_uv_script(uv_script_path).expect("Failed to read uv script");
    assert!(!uv_script.is_empty());
}

#[test]
fn test_find_latest_version() {
    let dependency = "serde";
    let latest_version = find_latest_version(dependency).expect("Failed to find latest version");
    assert!(latest_version > Version::parse("0.0.0").unwrap());
}

#[test]
fn test_upgrade_dependencies() {
    let pyproject_path = "tests/test_pyproject.toml";
    let mut pyproject = read_pyproject_toml(pyproject_path).expect("Failed to read pyproject.toml");
    upgrade_dependencies(&mut pyproject).expect("Failed to upgrade dependencies");

    if let Some(deps) = pyproject.get("dependencies").and_then(|d| d.as_table()) {
        for (name, version) in deps {
            let latest_version = find_latest_version(name).expect("Failed to find latest version");
            assert_eq!(version.as_str().unwrap(), latest_version.to_string());
        }
    }
}
