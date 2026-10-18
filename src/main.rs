//! Writes the PelaosTech site to an output directory (default `dist`).
//!
//! GitHub Pages serves static files only, so the "backend" runs at build time: every page
//! is rendered to complete HTML here, and the Dart front end (web/app.dart) only adds the
//! interactive extras.

mod pages;

use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let out = std::env::args().nth(1).unwrap_or_else(|| "dist".to_string());
    let out = Path::new(&out);
    if out.exists() {
        fs::remove_dir_all(out)?;
    }
    for file in pages::site() {
        let path = out.join(&file.path);
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        fs::write(&path, &file.contents)?;
        println!("{:>7} bytes  {}", file.contents.len(), path.display());
    }
    Ok(())
}
