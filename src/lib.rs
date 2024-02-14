#![feature(fs_try_exists)]
use std::{
    fs::{self, File},
    io::{self, Write},
};

use anyhow::{Context, Result};
use clippers::{Clipboard, ClipperData};

pub fn init(path: &String) -> Result<()> {
    if fs::try_exists(path).is_ok_and(|x| x) {
        println!("Already initialized!");
        return Ok(());
    }

    fs::create_dir(path)?;
    println!("Initialized sniptip at: {}", path);
    Ok(())
}

pub fn add_snip(path: &String, snip: &String) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(snip.as_bytes())?;
    println!("Sniptip saved!");

    Ok(())
}

pub fn add_snip_from_clipboard(path: &String) -> Result<()> {
    let mut clipboard = Clipboard::get();
    match clipboard.read() {
        Some(ClipperData::Text(text)) => {
            add_snip(path, &text.as_str().to_owned())
        }
        _ => {
            println!("No snip found in the clipboard!");
            Ok(())
        }
    }
}

pub fn query_snip(query: &String, path: &String) -> Result<()> {
    let file_names = get_file_names(path)
        .unwrap()
        .into_iter()
        .filter(|n| n.to_owned().contains(query))
        .collect::<Vec<_>>();

    if file_names.is_empty() {
        println!("No sniptips found!");
        return Ok(());
    }

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

pub fn list_snips(path: &String) -> Result<()> {
    let file_names = get_file_names(path)?;

    if file_names.is_empty() {
        println!("No sniptips found!");
        return Ok(());
    }

    io::stdout()
        .write_all(format!("{}\n", file_names.join("\n")).as_bytes())
        .with_context(|| format!("Could not read sniptips store at path: {}", path))?;

    Ok(())
}

pub fn delete_snip(path: &String) -> Result<()> {
    fs::remove_file(path)
        .with_context(|| format!("Could not remove sniptip located at: {}", path))?;

    println!("Sniptip deleted!");

    Ok(())
}

fn get_file_names(path: &String) -> Result<Vec<String>> {
    let dir = fs::read_dir(path)?;

    let paths = dir
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let file_names = paths
        .iter()
        .filter(|p| !p.is_dir())
        .map(|h| h.file_name().unwrap().to_str().unwrap().to_string())
        .collect::<Vec<_>>();

    Ok(file_names)
}
