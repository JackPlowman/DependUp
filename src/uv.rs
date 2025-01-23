use std::fs;
use std::error::Error;

pub fn read_uv_script(file_path: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;
    Ok(content)
}

pub fn extract_dependencies(uv_script: &str) -> Vec<String> {
    let mut dependencies = Vec::new();
    for line in uv_script.lines() {
        if line.starts_with("dependency:") {
            if let Some(dep) = line.split_whitespace().nth(1) {
                dependencies.push(dep.to_string());
            }
        }
    }
    dependencies
}
