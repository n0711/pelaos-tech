//! Writes the PelaosTech site to an output directory (default `dist`).
//!
//! GitHub Pages serves static files only, so the "backend" runs at build time: every page
//! is rendered to complete HTML here, and the Dart front end (web/app.dart) only adds the
//! interactive extras.

mod pages;
mod output;

use std::path::Path;

fn main() -> std::io::Result<()> {
    let out = std::env::args().nth(1).unwrap_or_else(|| "dist".to_string());
    let out = Path::new(&out);
    let files = pages::site();
    output::write_site(out, &files)?;
    for file in files {
        println!("{:>7} bytes  {}", file.contents.len(), out.join(&file.path).display());
    }
    Ok(())
}
