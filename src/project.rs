use std::fs;
use toml::Value;
use reqwest::blocking::get;
use semver::Version;
use serde_json;

pub fn extract_package_name(dep_str: &str) -> &str {
    dep_str.split(&['=', '>', '<', '~', '!'][..])
        .next()
        .unwrap_or(dep_str)
        .trim()
}

pub fn find_latest_version(dependency: &str) -> Result<Version, Box<dyn std::error::Error>> {
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
