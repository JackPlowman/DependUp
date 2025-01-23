use std::fs;
use std::collections::HashMap;
use toml::Value;
use reqwest::blocking::get;
use semver::Version;

fn read_pyproject_toml() -> HashMap<String, String> {
    let content = fs::read_to_string("pyproject.toml").expect("Failed to read pyproject.toml");
    let value: Value = content.parse().expect("Failed to parse pyproject.toml");
    let dependencies = value["tool"]["poetry"]["dependencies"].as_table().expect("Failed to get dependencies");
    dependencies.iter().map(|(k, v)| (k.clone(), v.as_str().unwrap().to_string())).collect()
}

fn find_latest_version(dependency: &str) -> String {
    let url = format!("https://pypi.org/pypi/{}/json", dependency);
    let response = get(&url).expect("Failed to fetch dependency info");
    let json: Value = response.json().expect("Failed to parse dependency info");
    json["info"]["version"].as_str().unwrap().to_string()
}

fn upgrade_dependencies(dependencies: &mut HashMap<String, String>) {
    for (name, version) in dependencies.iter_mut() {
        let latest_version = find_latest_version(name);
        if Version::parse(&latest_version).unwrap() > Version::parse(version).unwrap() {
            *version = latest_version;
        }
    }
}

fn main() {
    let mut dependencies = read_pyproject_toml();
    upgrade_dependencies(&mut dependencies);
    println!("Updated dependencies: {:?}", dependencies);
}
