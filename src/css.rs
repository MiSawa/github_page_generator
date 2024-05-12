use anyhow::{anyhow, Result};
use camino::Utf8Path as Path;

pub fn compile(file: impl AsRef<Path>) -> Result<Vec<u8>> {
    let option = sass_rs::Options {
        output_style: sass_rs::OutputStyle::Compressed,
        indented_syntax: false,
        include_paths: vec![],
        ..sass_rs::Options::default()
    };
    let css = sass_rs::compile_file(file.as_ref(), option)
        .map_err(|e| anyhow!("Failed to compile sass: {e}"))?;
    Ok(css.into())
}
pub fn compile2(root: &Path, file: &Path) -> Result<Vec<u8>> {
    let file = root.join(file);
    let option = sass_rs::Options {
        output_style: sass_rs::OutputStyle::Compressed,
        indented_syntax: false,
        include_paths: vec![],
        ..sass_rs::Options::default()
    };
    let css =
        sass_rs::compile_file(file, option).map_err(|e| anyhow!("Failed to compile sass: {e}"))?;
    Ok(css.into())
}
