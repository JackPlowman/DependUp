use std::fs;
use toml::Value;

pub fn read_pyproject_toml(file_path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let value = content.parse::<Value>()?;
    Ok(value)
}

pub fn extract_dependencies(pyproject: &Value) -> Option<&Value> {
    pyproject.get("tool").and_then(|tool| tool.get("poetry").and_then(|poetry| poetry.get("dependencies")))
}
