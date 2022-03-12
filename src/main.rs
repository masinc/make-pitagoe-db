#[macro_use]
extern crate log;

use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::io::{prelude::*, BufWriter};
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    /// Talker name (e.g. "鶴巻マキ")
    #[clap(short, long)]
    name: String,

    /// Talker description
    #[clap(short, long)]
    description: Option<String>,

    /// Pitagoe path
    #[clap(short, long)]
    path: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    /// The relative path to the file.
    relative_path: String,
    /// The Name
    name: String,
    /// セリフ in Japanese
    line: String,
    /// 読み方 in Japanese (Only カタカナ)
    pronunciation: String,
    /// The sound category
    category: String,
}

/// Supported file extensions
const SOUND_EXTENSIONS: &[&str] = &["wav"];

fn get_sound_items(dir: &Path) -> Result<Vec<PathBuf>> {
    let pattern = format!(
        r"{}\**\*.{}",
        dir.to_str().unwrap(),
        SOUND_EXTENSIONS.join("|")
    );

    let mut result = Vec::new();

    for f in globwalk::glob(pattern)?.into_iter() {
        let f = f?;
        result.push(f.into_path());
    }

    Ok(result)
}

fn parse(path: impl AsRef<Path>) -> Result<Vec<Record>> {
    let base_path = path.as_ref();

    let mut records = Vec::new();
    let items = get_sound_items(&base_path)?;

    for item in items {
        let relative_path = item.strip_prefix(base_path)?;
        let relative_path_str = relative_path.to_str().map(|s| s.to_string());

        let name = item
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string());
        let line = name.clone();
        let pronunciation = "".into();
        let category: Option<String> = {
            let components: Option<Vec<_>> = relative_path
                .components()
                .map(|c| c.as_os_str().to_str())
                .collect();
            components.map(|v| v[0..v.len() - 1].join("/"))
        };

        match (relative_path_str, name, line, pronunciation, category) {
            (Some(relative_path), Some(name), Some(line), pronunciation, Some(category)) => {
                records.push(Record {
                    relative_path,
                    name,
                    line,
                    pronunciation,
                    category,
                });
            }
            _ => {}
        }
    }

    Ok(records)
}

fn create_csv(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    let records = parse(path)?;

    let csv_path = path.join(path.file_name().unwrap()).with_extension("csv");

    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_path(&csv_path)?;

    for record in records {
        writer.serialize(record)?;
    }

    info!("Created {}", csv_path.display());
    Ok(())
}

fn create_ini(
    path: impl AsRef<Path>,
    name: impl Into<String>,
    description: impl Into<String>,
) -> Result<()> {
    let path = path.as_ref();
    let ini_path = path.join("character.ini");

    let mut f = fs::File::create(&ini_path)?;
    let mut writer = BufWriter::new(&mut f);

    writeln!(writer, "NAME\t{}", name.into())?;
    writeln!(writer, "DESC\t{}", description.into())?;

    info!("Created {}", ini_path.display());

    Ok(())
}

fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let args = Args::parse();

    create_csv(&args.path)?;
    create_ini(&args.path, args.name, args.description.unwrap_or_default())?;
    Ok(())
}
