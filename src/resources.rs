use anyhow::Result;
use camino::{Utf8Path as Path, Utf8PathBuf as PathBuf};

use crate::Context;

pub enum Resource {
    Asset(Box<dyn Asset>),
    Page(Box<dyn Page>),
}
pub trait Asset {
    fn url(&self) -> &Path;
    fn render(&self, context: &mut Context) -> Result<Vec<u8>>;
}
pub trait Page {
    fn url(&self) -> &Path;
    fn title(&self) -> &str;
    fn tags(&self) -> Vec<String>;
    fn render(&self, context: &mut Context) -> Result<Vec<u8>>;
}

impl Resource {
    pub fn from_asset(asset: impl 'static + Asset) -> Self {
        Self::Asset(Box::new(asset))
    }
    pub fn from_page(page: impl 'static + Page) -> Self {
        Self::Page(Box::new(page))
    }
    pub fn url(&self) -> &Path {
        match self {
            Resource::Asset(asset) => asset.url(),
            Resource::Page(page) => page.url(),
        }
    }
    pub fn render(&self, context: &mut Context) -> Result<Vec<u8>> {
        match self {
            Resource::Asset(asset) => asset.render(context),
            Resource::Page(page) => page.render(context),
        }
    }
}

pub struct RawAsset {
    path: PathBuf,
    content: Vec<u8>,
}
impl RawAsset {
    pub fn new(path: impl AsRef<Path>, content: Vec<u8>) -> Self {
        Self {
            path: path.as_ref().into(),
            content,
        }
    }
}
impl Asset for RawAsset {
    fn url(&self) -> &Path {
        &self.path
    }

    fn render(&self, _context: &mut crate::Context) -> Result<Vec<u8>> {
        Ok(self.content.clone())
    }
}
