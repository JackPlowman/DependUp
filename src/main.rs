use std::fs;
use std::io::{BufRead, BufReader, Write};
use indexmap::IndexMap;

mod project;

fn upgrade_dependencies(pyproject_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::open(pyproject_path)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
    let mut in_dependencies_section = false;
    let mut dependencies: IndexMap<String, String> = IndexMap::new();

    // First, extract dependencies and their original order
    let mut start_index = None;
    let mut end_index = None;
    for (i, line) in lines.iter().enumerate() {
        if line.trim() == "[project.dependencies]" {
            in_dependencies_section = true;
            start_index = Some(i);
            continue;
        }

        if in_dependencies_section {
            if line.trim().is_empty() || line.starts_with('[') {
                in_dependencies_section = false;
                end_index = Some(i);
                break;
            }

            if let Some((name, version)) = line.split_once("==") {
                dependencies.insert(name.trim().to_string(), version.trim().to_string());
            }
        }
    }

    // Upgrade versions
    let mut upgraded_dependencies: IndexMap<String, String> = IndexMap::new();
    for (name, _) in dependencies.iter() {
        println!("Upgrading dependency: {}", name);
        let latest_version = project::find_latest_version(name)?;
        upgraded_dependencies.insert(name.clone(), format!("=={}", latest_version));
    }

    // Modify the lines in memory
    if let (Some(start), Some(end)) = (start_index, end_index) {
        let mut insert_index = start + 1;
        for (name, version) in upgraded_dependencies.iter() {
            lines[insert_index] = format!("{} {}", name, version);
            insert_index += 1;
        }
        // Remove old dependency lines
        lines.splice(start + 1..end, upgraded_dependencies.iter().map(|(name, version)| format!("{} {}", name, version)));
    }

    // Write back to file
    let mut file = fs::File::create(pyproject_path)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }

    println!("Updated pyproject.toml successfully");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pyproject_path = "pyproject.toml";
    upgrade_dependencies(pyproject_path)?;

    Ok(())
}
