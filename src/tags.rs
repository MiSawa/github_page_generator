use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;
use itertools::Itertools as _;
use serde::Serialize;

use crate::resources::Page;

#[derive(Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Doc {
    title: String,
    url: PathBuf,
}
pub struct TagRepository {
    index: HashMap<String, Vec<Doc>>,
}

impl Default for TagRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl TagRepository {
    pub fn new() -> Self {
        Self {
            index: Default::default(),
        }
    }
    pub fn track(&mut self, resource: &dyn Page) {
        for tag in resource.tags() {
            self.index.entry(tag).or_default().push(Doc {
                url: resource.url().into(),
                title: resource.title().into(),
            });
        }
    }
    pub fn resources_with_tag(&self, tag: &str) -> Option<Vec<Doc>> {
        self.index
            .get(tag)
            .map(|docs| docs.iter().cloned().sorted().collect())
    }
    pub fn count_for_tag(&self, tag: &str) -> Option<usize> {
        self.index.get(tag).map(Vec::len)
    }
    pub fn all_tags(&self) -> Vec<String> {
        self.index.keys().cloned().sorted().collect()
    }
    pub fn get_tag_url(&self, tag: &str) -> Result<String> {
        anyhow::ensure!(self.index.contains_key(tag), "Tag {tag} doesn't exist");
        Ok(format!("/tags/{tag}.html"))
    }
}
