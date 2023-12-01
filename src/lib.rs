use std::{
    fs::{self, File},
    io::{self, Write},
};

use anyhow::{Context, Result};

pub fn init(path: &String) -> Result<()> {
    fs::create_dir(path)?;
    Ok(())
}

pub fn add_snip(path: &String, snip: &String) -> Result<()> {
    let mut file = File::create(path)?;

    file.write_all(snip.as_bytes())?;

    println!("Sniptip saved!");

    Ok(())
}

pub fn query_snip(query: &String, path: &String) -> Result<()> {
    let dir = fs::read_dir(path)?;

    let paths = dir
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let file_names = paths
        .iter()
        .filter(|p| !p.is_dir())
        .map(|h| h.file_name().unwrap().to_str().unwrap())
        // TODO(anfelo): use a better filter, like a fuzzy finder
        .filter(|n| n.contains(query))
        .collect::<Vec<_>>();

    io::stdout().write_all(format!("{}\n", file_names.join("\n")).as_bytes())?;

    Ok(())
}

pub fn show_snip(path: &String) -> Result<()> {
    let contents = fs::read_to_string(path)?;

    io::stdout()
        .write_all(contents.as_bytes())
        .with_context(|| format!("Could not find sniptip located at: {}", path))?;

    Ok(())
}

pub fn delete_snip(path: &String) -> Result<()> {
    fs::remove_file(path)
        .with_context(|| format!("Could not remove sniptip located at: {}", path))?;

    println!("Sniptip deleted!");

    Ok(())
}
