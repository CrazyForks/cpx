use std::path::{Path, PathBuf};
use tokio::io;

pub fn prompt_overwrite(path: &Path) -> io::Result<bool> {
    use std::io::{Write, stdin, stdout};

    print!("overwrite '{}'? (y/n): ", path.display());
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    Ok(input.trim().eq_ignore_ascii_case("y"))
}

pub fn with_parents(dest: &Path, source: &Path) -> PathBuf {
    let skip_count = if source.is_absolute() { 1 } else { 0 };
    let components = source.components().skip(skip_count);

    let mut relative = PathBuf::new();
    for comp in components {
        relative.push(comp.as_os_str());
    }

    dest.join(relative)
}
