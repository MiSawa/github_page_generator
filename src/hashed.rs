use std::collections::HashMap;

use anyhow::{anyhow, ensure, Result};
use camino::{Utf8Path as Path, Utf8PathBuf as PathBuf};

use crate::resources::RawAsset;

#[derive(Default)]
pub struct HashedResources {
    index: HashMap<String, PathBuf>,
}
impl HashedResources {
    pub(crate) fn add(&mut self, path: impl AsRef<Path>, content: Vec<u8>) -> Result<RawAsset> {
        let path = path.as_ref();
        let hash = &crate::utils::hexsum(&content)[..8];

        let stem = path.file_stem().ok_or_else(|| anyhow!("No file name?"))?;
        let real_path = if let Some(ext) = path.extension() {
            path.with_file_name(format!("{stem}-{hash}.{ext}"))
        } else {
            path.with_file_name(format!("{stem}-{hash}"))
        };
        let existing = self.index.insert(path.to_string(), real_path.clone());
        ensure!(existing.is_none(), "Hashed resource {path} already exists");
        Ok(RawAsset::new(real_path, content))
    }

    pub(crate) fn lookup_output_path(&self, name: &str) -> Option<&Path> {
        self.index.get(name).map(|path| path.as_ref())
    }
}
