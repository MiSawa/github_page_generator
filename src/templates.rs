use anyhow::Result;
use camino::{Utf8Path as Path, Utf8PathBuf as PathBuf};

use crate::{resources::Page, value, Context, Value};

pub struct TemplatedPage {
    url: PathBuf,
    title: String,
    template_name: String,
    values: Value,
}
impl TemplatedPage {
    pub fn create(
        context: &mut Context,
        template_name: &str,
        url: impl AsRef<Path>,
        title_template: &str,
        values: Value,
    ) -> Result<Self> {
        let title = context.handlebars.render_template(
            title_template,
            &serde_merge::mmerge(&context.global_values, &values)?,
        )?;
        let values = serde_merge::tmerge(&values, value!({ "title": title }))?;
        Ok(Self {
            url: url.as_ref().into(),
            title,
            template_name: template_name.into(),
            values,
        })
    }
}
impl Page for TemplatedPage {
    fn url(&self) -> &Path {
        &self.url
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn tags(&self) -> Vec<String> {
        self.values
            .as_object()
            .and_then(|o| o.get("tags"))
            .and_then(|o| o.as_array())
            .into_iter()
            .flat_map(|v| v.iter())
            .flat_map(|v| v.as_str())
            .map(|v| v.to_string())
            .collect()
    }

    fn render(&self, context: &mut Context) -> Result<Vec<u8>> {
        let data = serde_merge::mmerge(&context.global_values, &self.values)?;
        let rendered = context.handlebars.render(&self.template_name, &data)?;
        Ok(rendered.into())
    }
}
