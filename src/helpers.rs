use std::sync::{Arc, Mutex};

use handlebars::{
    to_json, Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderErrorReason,
};
use itertools::Itertools as _;

use crate::{hashed::HashedResources, tags::TagRepository};

pub fn register_helpers(
    handlebars: &mut Handlebars,
    top_page: &str,
    tag: Arc<Mutex<TagRepository>>,
    hashed: Arc<Mutex<HashedResources>>,
) {
    let mut top_page = top_page.to_string();
    if !top_page.ends_with('/') {
        top_page.push('/');
    }
    handlebars.register_helper("join", Box::new(Join));
    handlebars.register_helper("pages_with_tag", Box::new(PagesWithTag(tag.clone())));
    handlebars.register_helper("all_tags", Box::new(AllTags(tag.clone())));
    handlebars.register_helper("link_to", Box::new(LinkTo(top_page.clone())));
    handlebars.register_helper("link_to_tag", Box::new(LinkToTag(top_page.clone(), tag)));
    handlebars.register_helper(
        "resolve_hash",
        Box::new(ResolveHash(top_page.clone(), hashed)),
    );
}

fn resolve_link_and_encode(top_page: &str, url: &str) -> String {
    if url.starts_with("./") {
        url_escape::encode_path(&url).to_string()
    } else {
        let url_no_slash = url.strip_prefix('/').unwrap_or(url);
        let url = top_page.to_owned() + url_no_slash;
        url_escape::encode_path(&url).to_string()
    }
}

struct Join;
impl handlebars::HelperDef for Join {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        helper: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<handlebars::ScopedJson<'rc>, handlebars::RenderError> {
        let values = helper
            .param(0)
            .ok_or(RenderErrorReason::ParamNotFoundForIndex("join", 0))?
            .value()
            .as_array()
            .ok_or(RenderErrorReason::InvalidParamType("array"))?;
        let joined = values
            .iter()
            .map(|s| match s {
                serde_json::Value::String(s) => s.to_string(),
                _ => s.to_string(),
            })
            .join(", ");
        Ok(handlebars::ScopedJson::Derived(joined.into()))
    }
}

struct LinkTo(String);
impl handlebars::HelperDef for LinkTo {
    fn call<'reg: 'rc, 'rc>(
        &self,
        helper: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let title = helper
            .param(0)
            .map(|v| v.render())
            .ok_or(RenderErrorReason::ParamNotFoundForIndex("link_to", 0))?;
        let url = helper
            .param(1)
            .map(|v| v.render())
            .ok_or(RenderErrorReason::ParamNotFoundForIndex("link_to", 1))?;
        let url = resolve_link_and_encode(&self.0, &url);
        let title = handlebars::html_escape(&title);
        write!(out, r#"<a href="{url}">{title}</a>"#)?;
        Ok(())
    }
}

struct AllTags(Arc<Mutex<TagRepository>>);

impl handlebars::HelperDef for AllTags {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        _: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<handlebars::ScopedJson<'rc>, handlebars::RenderError> {
        let tags = self.0.lock().unwrap().all_tags();
        Ok(handlebars::ScopedJson::Derived(to_json(tags)))
    }
}

struct PagesWithTag(Arc<Mutex<TagRepository>>);

impl handlebars::HelperDef for PagesWithTag {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        helper: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<handlebars::ScopedJson<'rc>, handlebars::RenderError> {
        let name = helper
            .param(0)
            .ok_or(RenderErrorReason::ParamNotFoundForIndex(
                "pages_with_tag",
                0,
            ))?
            .render();
        let resources = self
            .0
            .lock()
            .unwrap()
            .resources_with_tag(&name)
            .unwrap_or_default();
        Ok(handlebars::ScopedJson::Derived(to_json(resources)))
    }
}

struct LinkToTag(String, Arc<Mutex<TagRepository>>);

impl handlebars::HelperDef for LinkToTag {
    fn call<'reg: 'rc, 'rc>(
        &self,
        helper: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let name = helper
            .param(0)
            .ok_or(RenderErrorReason::ParamNotFoundForIndex("link_to_tag", 0))?
            .render();
        let url = self
            .1
            .lock()
            .unwrap()
            .get_tag_url(&name)
            .map_err(|e| handlebars::RenderErrorReason::NestedError(e.into()))?;
        let len = self.1.lock().unwrap().count_for_tag(&name).unwrap();
        let url = resolve_link_and_encode(&self.0, &url);
        let title = handlebars::html_escape(&name);
        write!(out, r#"<a href="{url}">{title} ({len})</a>"#)?;
        Ok(())
    }
}

struct ResolveHash(String, Arc<Mutex<HashedResources>>);

impl handlebars::HelperDef for ResolveHash {
    fn call<'reg: 'rc, 'rc>(
        &self,
        helper: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let name = helper
            .param(0)
            .ok_or(RenderErrorReason::ParamNotFoundForIndex("resolve_hash", 0))?
            .render();
        let locked = self.1.lock().unwrap();
        let url = locked
            .lookup_output_path(&name)
            .ok_or_else(|| {
                handlebars::RenderErrorReason::Other(format!(
                    "Hashed resource with {name} doesn't exist"
                ))
            })?
            .as_str();
        let url = resolve_link_and_encode(&self.0, url);
        write!(out, "{url}")?;
        Ok(())
    }
}
