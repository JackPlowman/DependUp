use std::fs;
use toml::Value;
use semver::Version;
use indexmap::IndexMap;

mod project;

fn upgrade_dependencies(project: &mut Value) -> Result<(), Box<dyn std::error::Error>> {
    let dependencies = project.get_mut("project")
        .and_then(|p| p.get_mut("dependencies"))
        .ok_or("No project.dependencies found")?;

    match dependencies {
        Value::Array(deps) => {
            for dep in deps.iter_mut() {
                if let Some(dep_str) = dep.as_str() {
                    let package_name = project::extract_package_name(dep_str);
                    println!("Upgrading dependency: {}", package_name);
                    let latest_version = project::find_latest_version(dep_str)?;
                    *dep = Value::String(format!("{}=={}", package_name, latest_version));
                }
            }
        },
        Value::Table(deps) => {
            // Convert to IndexMap to preserve order
            let mut index_map: IndexMap<String, Value> = deps.clone().into_iter().collect();
            for (name, version) in index_map.iter_mut() {
                println!("Upgrading dependency: {}", name);
                let latest_version = project::find_latest_version(name)?;
                *version = Value::String(format!("=={}", latest_version));
            }
            // Repopulate the original table with the updated IndexMap
            *deps = index_map.into_iter().collect();
        },
        _ => return Err("Dependencies must be an array or table".into()),
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pyproject_path = "pyproject.toml";
    let mut pyproject = project::read_pyproject_toml(pyproject_path)?;

    upgrade_dependencies(&mut pyproject)?;

    // Write back to file
    fs::write(pyproject_path, toml::to_string(&pyproject)?)?;
    println!("Updated pyproject.toml successfully");

    Ok(())
}
