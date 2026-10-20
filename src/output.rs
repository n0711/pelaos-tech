//! Stage complete output before replacing a directory owned by this generator.
use crate::pages::File;
use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

const MARKER: &str = ".pelaos-site-output";
const SIGNATURE: &str = "pelaos-site generated output v1\n";
static NEXT: AtomicU64 = AtomicU64::new(0);

fn invalid(message: &str) -> io::Error {
    io::Error::new(ErrorKind::InvalidInput, message)
}

fn no_symlinks(path: &Path) -> io::Result<()> {
    for ancestor in path.ancestors() {
        match fs::symlink_metadata(ancestor) {
            Ok(meta) if meta.file_type().is_symlink() => {
                return Err(invalid("output path must not contain symbolic links"));
            }
            Ok(_) => {}
            Err(e) if e.kind() == ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn check_output(out: &Path) -> io::Result<()> {
    no_symlinks(out)?;
    if !out.exists() {
        return Ok(());
    }
    if !out.is_dir() {
        return Err(invalid("output must be a directory"));
    }
    if fs::read_dir(out)?.next().is_none() {
        return Ok(());
    }
    let marker = out.join(MARKER);
    no_symlinks(&marker)?;
    if fs::read_to_string(marker).ok().as_deref() != Some(SIGNATURE) {
        return Err(invalid("refusing to replace a nonempty directory without a PELAOS output marker; choose a new empty output directory"));
    }
    Ok(())
}

fn vacant_sibling(parent: &Path, label: &str) -> io::Result<PathBuf> {
    for _ in 0..100 {
        let id = NEXT.fetch_add(1, Ordering::Relaxed);
        let path = parent.join(format!(".pelaos-{label}-{}-{id}", std::process::id()));
        match fs::create_dir(&path) {
            Ok(()) => return Ok(path),
            Err(e) if e.kind() == ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e),
        }
    }
    Err(io::Error::new(ErrorKind::AlreadyExists, "cannot reserve staging directory"))
}

pub fn write_site(out: &Path, files: &[File]) -> io::Result<()> {
    // Reject `..` before checking ancestors, and reject filesystem roots.
    let absolute = std::path::absolute(out)?;
    if absolute.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err(invalid("output path must not contain '..'"));
    }
    if absolute.file_name().is_none() {
        return Err(invalid("output must not be a filesystem root"));
    }
    check_output(&absolute)?;
    let parent = absolute.parent().ok_or_else(|| invalid("missing output parent"))?;
    fs::create_dir_all(parent)?;
    let stage = vacant_sibling(parent, "stage")?;
    let staged = (|| {
        for file in files {
            let relative = Path::new(&file.path);
            if file.path.is_empty()
                || relative.components().any(|c| !matches!(c, Component::Normal(_)))
                || relative == Path::new(MARKER)
            {
                return Err(invalid("generated file path must be relative and stay inside output"));
            }
            let path = stage.join(relative);
            fs::create_dir_all(path.parent().unwrap())?;
            fs::write(path, &file.contents)?;
        }
        fs::write(stage.join(MARKER), SIGNATURE)
    })();
    if let Err(e) = staged {
        let _ = fs::remove_dir_all(&stage);
        return Err(e);
    }
    // Recheck immediately before replacement. This is a single-writer build tool.
    if let Err(e) = check_output(&absolute) {
        let _ = fs::remove_dir_all(&stage);
        return Err(e);
    }
    if !absolute.exists() {
        if let Err(e) = fs::rename(&stage, &absolute) {
            let _ = fs::remove_dir_all(&stage);
            return Err(e);
        }
        return Ok(());
    }
    let backup = match vacant_sibling(parent, "backup") {
        Ok(path) => path,
        Err(e) => {
            let _ = fs::remove_dir_all(&stage);
            return Err(e);
        }
    };
    if let Err(e) = fs::remove_dir(&backup) {
        let _ = fs::remove_dir_all(&stage);
        return Err(e);
    }
    if let Err(e) = fs::rename(&absolute, &backup) {
        let _ = fs::remove_dir_all(&stage);
        return Err(e);
    }
    if let Err(e) = fs::rename(&stage, &absolute) {
        let restored = fs::rename(&backup, &absolute);
        let _ = fs::remove_dir_all(&stage);
        return match restored {
            Ok(()) => Err(e),
            Err(restore) => Err(io::Error::other(format!("publish failed: {e}; restore failed: {restore}; previous output remains at {}", backup.display()))),
        };
    }
    fs::remove_dir_all(backup)
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Temp(PathBuf);
    impl Temp {
        fn new() -> Self {
            Self(vacant_sibling(&std::env::temp_dir(), "test").unwrap())
        }
    }
    impl Drop for Temp {
        fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); }
    }
    fn file(path: &str, contents: &str) -> File {
        File { path: path.into(), contents: contents.into() }
    }
    #[test]
    fn refuses_unowned_output_without_modifying_it() {
        let tmp = Temp::new();
        fs::write(tmp.0.join("important.txt"), "keep").unwrap();
        assert!(write_site(&tmp.0, &[file("index.html", "new")]).is_err());
        assert_eq!(fs::read_to_string(tmp.0.join("important.txt")).unwrap(), "keep");
    }
    #[test]
    fn creates_and_replaces_owned_output() {
        let tmp = Temp::new();
        let out = tmp.0.join("dist");
        write_site(&out, &[file("old.html", "old")]).unwrap();
        write_site(&out, &[file("nested/index.html", "new")]).unwrap();
        assert!(!out.join("old.html").exists());
        assert_eq!(fs::read_to_string(out.join("nested/index.html")).unwrap(), "new");
    }
    #[test]
    fn failed_staging_preserves_previous_build() {
        let tmp = Temp::new();
        let out = tmp.0.join("dist");
        write_site(&out, &[file("index.html", "old")]).unwrap();
        assert!(write_site(&out, &[file("index.html", "new"), file("../escape", "bad")]).is_err());
        assert_eq!(fs::read_to_string(out.join("index.html")).unwrap(), "old");
        assert!(!tmp.0.join("escape").exists());
        assert_eq!(fs::read_dir(&tmp.0).unwrap().count(), 1);
    }
    #[test]
    fn rejects_file_as_output() {
        let tmp = Temp::new();
        let out = tmp.0.join("file");
        fs::write(&out, "keep").unwrap();
        assert!(write_site(&out, &[]).is_err());
        assert_eq!(fs::read_to_string(out).unwrap(), "keep");
    }
    #[cfg(unix)]
    #[test]
    fn refuses_symlink_output_and_parent() {
        let tmp = Temp::new();
        let real = tmp.0.join("real");
        fs::create_dir(&real).unwrap();
        let link = tmp.0.join("link");
        std::os::unix::fs::symlink(&real, &link).unwrap();
        assert!(write_site(&link, &[]).is_err());
        assert!(write_site(&link.join("dist"), &[]).is_err());
        assert_eq!(fs::read_dir(real).unwrap().count(), 0);
    }
}
