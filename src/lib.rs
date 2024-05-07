use std::{
    fs,
    sync::{Arc, Mutex},
};

use anyhow::{anyhow, bail, Context as _, Result};
use camino::{Utf8Path as Path, Utf8PathBuf as PathBuf};
use handlebars::Handlebars;
use hashed::HashedResources;
use itertools::Itertools as _;
use tempfile::tempdir_in;
use wax::Pattern as _;

use crate::{resources::Resource, tags::TagRepository};

pub mod css;
pub mod frontmatter;
pub mod hashed;
pub mod helpers;
pub mod highlight;
pub mod markdown;
pub mod resources;
pub mod tags;
pub mod templates;
pub mod utils;

pub type Value = serde_json::Value;
pub use serde_json::json as value;

pub struct Context<'a> {
    pub handlebars: Handlebars<'a>,
    pub global_values: Value,
    pub tag: Arc<Mutex<TagRepository>>,
    pub hashed: Arc<Mutex<HashedResources>>,
}
impl<'a> Context<'a> {
    fn new(base_url: &str, global_values: Value) -> Self {
        let mut handlebars = Handlebars::new();
        let tag: Arc<Mutex<TagRepository>> = Default::default();
        let hashed: Arc<Mutex<HashedResources>> = Default::default();
        helpers::register_helpers(&mut handlebars, base_url, tag.clone(), hashed.clone());
        Self {
            handlebars,
            global_values,
            tag,
            hashed,
        }
    }
}

pub struct SSBuilder<'a> {
    context: Context<'a>,
    resources: Vec<Resource>,
}
impl<'a> SSBuilder<'a> {
    pub fn new(base_url: &str, global_values: Value) -> Self {
        Self {
            context: Context::new(base_url, global_values),
            resources: Default::default(),
        }
    }
    pub fn context(&mut self) -> &mut Context<'a> {
        &mut self.context
    }
    pub fn add(&mut self, resource: Resource) {
        if let Resource::Page(page) = &resource {
            self.context.tag.lock().unwrap().track(page.as_ref());
        }
        self.resources.push(resource);
    }
    pub fn add_hashed_asset(&mut self, path: impl AsRef<Path>, content: Vec<u8>) -> Result<()> {
        let asset = self.context.hashed.lock().unwrap().add(path, content)?;
        self.resources.push(Resource::from_asset(asset));
        Ok(())
    }

    pub fn compile(&mut self, output_dir: impl AsRef<Path>) -> Result<()> {
        fn output(context: &mut Context, dir: &Path, resource: &Resource) -> Result<()> {
            let path = resource.url();
            let path = dir.join(path.strip_prefix("/").unwrap_or(path));
            if let Some(p) = path.parent() {
                fs::create_dir_all(p)?;
            }
            let rendered = resource
                .render(context)
                .with_context(|| format!("Failed to render {:?}", resource.url()))?;
            fs::write(path, rendered)
                .with_context(|| format!("Failed to write {:?}", resource.url()))?;
            Ok(())
        }
        if let Some((_, p)) = self
            .resources
            .iter()
            .map(|r| r.url())
            .dedup_with_count()
            .find(|&(c, _)| c > 1)
        {
            bail!("Resource {p} added more than once");
        }

        let out_parent = output_dir
            .as_ref()
            .parent()
            .ok_or_else(|| anyhow!("unable to get parent directory of the output directory"))?;
        let temp_dir = tempdir_in(out_parent)?;
        let temp_dir_path = temp_dir.path().try_into()?;
        for page in &self.resources {
            output(&mut self.context, temp_dir_path, page)?;
        }
        let dir = temp_dir.into_path();
        if let Err(e) = fs::remove_dir_all(output_dir.as_ref()) {
            if e.kind() != std::io::ErrorKind::NotFound {
                bail!(e);
            }
        }
        fs::create_dir(output_dir.as_ref())?;
        fs::rename(dir, output_dir.as_ref())?;
        Ok(())
    }
}

type Handler<'a> = Box<dyn 'a + Fn(&mut SSBuilder, wax::MatchedText) -> Result<()>>;
pub struct SourceTree<'a> {
    root: PathBuf,
    routes: Vec<(wax::Glob<'static>, Handler<'a>)>,
}
impl<'a> SourceTree<'a> {
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            routes: vec![],
        }
    }
    pub fn route(
        &mut self,
        glob: &'static str,
        handler: impl 'a + Fn(&mut SSBuilder, wax::MatchedText) -> Result<()>,
    ) {
        let glob = wax::Glob::new(glob).unwrap();
        self.routes.push((glob, Box::new(handler)));
    }
    pub fn complete(self, builder: &mut SSBuilder) -> Result<()> {
        for entry in wax::Glob::new("**/*").unwrap().walk(self.root) {
            let entry = entry?;
            let candidate = entry.to_candidate_path();
            for (glob, handler) in &self.routes {
                if let Some(matched) = glob.matched(&candidate) {
                    handler(builder, matched)?;
                }
            }
        }
        Ok(())
    }
}
