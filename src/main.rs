use std::fs;

use anyhow::Result;
use camino::{Utf8Path as Path, Utf8PathBuf as PathBuf};
use clap::Parser;
use handlebars::Handlebars;
use mssg::{
    css, frontmatter,
    markdown::MarkdownDocument,
    resources::{RawAsset, Resource},
    templates,
    utils::hexsum,
    value, SSBuilder, SourceTree,
};

fn configure_handlebars(handlebars: &mut Handlebars) -> Result<()> {
    handlebars.set_strict_mode(true);
    handlebars.register_templates_directory("./templates", Default::default())?;
    handlebars.set_prevent_indent(true);
    Ok(())
}

fn register_raw_file(
    builder: &mut SSBuilder,
    source_path: impl AsRef<Path>,
    url: impl AsRef<Path>,
) -> Result<()> {
    let content = fs::read(source_path.as_ref())?;
    builder.add(Resource::from_asset(RawAsset::new(url, content)));
    Ok(())
}

fn register_stylesheet(
    builder: &mut SSBuilder,
    source_path: impl AsRef<Path>,
    url: impl AsRef<Path>,
) -> Result<()> {
    let content = css::compile(source_path)?;
    builder.add_hashed_asset(url, content)?;
    Ok(())
}

fn register_markdown_page(
    builder: &mut SSBuilder,
    template: &str,
    source_path: impl AsRef<Path>,
    url: impl AsRef<Path>,
    title_template: Option<&str>,
) -> Result<()> {
    let markdown = MarkdownDocument::new(source_path)?;
    let html = markdown.render_html()?;
    let values = serde_merge::tmerge(
        markdown.frontmatter.unwrap_or(value!({})),
        value!({
            "content": html,
        }),
    )?;
    let page = templates::TemplatedPage::create(
        builder.context(),
        template,
        url,
        title_template.unwrap_or("{{title}}"),
        values,
    )?;
    builder.add(Resource::from_page(page));
    Ok(())
}

fn register_direct_handlebar_page(
    builder: &mut SSBuilder,
    source_path: impl AsRef<Path>,
    url: impl AsRef<Path>,
) -> Result<()> {
    let content = fs::read_to_string(source_path.as_ref())?;
    let (frontmatter, content) = frontmatter::split_and_parse_frontmatter(&content)?;
    let values = frontmatter.unwrap_or_else(|| value!({}));
    let name = hexsum(content);
    builder
        .context()
        .handlebars
        .register_template_string(&name, content)?;
    let page =
        templates::TemplatedPage::create(builder.context(), &name, url, "{{title}}", values)?;
    builder.add(Resource::from_page(page));
    Ok(())
}

fn register_tag_pages(builder: &mut SSBuilder) -> Result<()> {
    let tags = builder.context().tag.clone();
    let tags = tags.lock().unwrap();
    let mut tag_pages = vec![];
    for tag in &tags.all_tags() {
        let url = tags.get_tag_url(tag)?;
        let url = url.strip_prefix('/').unwrap_or(&url);
        let resource = templates::TemplatedPage::create(
            builder.context(),
            "tag",
            url,
            "Tag {{tag}}",
            value!({ "tag": tag }),
        )?;
        tag_pages.push(resource);
    }
    drop(tags);
    for page in tag_pages {
        builder.add(Resource::from_page(page));
    }
    Ok(())
}

/// My Static Site Generator
#[derive(Parser, Debug)]
struct Args {
    /// Base URL of the location you want to put the generated site to.
    #[arg(long, default_value = "/")]
    url_base: String,

    /// Directory containing source data.
    #[arg(long, default_value = "./source")]
    source: String,

    /// Directory to put the generated site.
    #[arg(long, default_value = "./build")]
    target: String,

    /// File containing global values for template interpolation.
    #[arg(long)]
    global_values: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let source_root: PathBuf = fs::canonicalize(args.source)?.try_into()?;
    let mut source_tree = SourceTree::new(source_root.clone());
    let global_values = if let Some(file) = args.global_values {
        let file = fs::File::open(file)?;
        serde_yaml::from_reader(std::io::BufReader::new(file))?
    } else {
        value!({})
    };

    let mut builder = SSBuilder::new(&args.url_base, global_values);
    configure_handlebars(&mut builder.context().handlebars)?;

    source_tree.route("**/*.css", |builder, matched| {
        register_stylesheet(
            builder,
            source_root.join(matched.complete()),
            matched.complete(),
        )
    });
    source_tree.route("{**/*.css}.scss", |builder, matched| {
        register_stylesheet(
            builder,
            source_root.join(matched.complete()),
            matched.get(1).unwrap(),
        )
    });
    source_tree.route("{README.md,**/*.{cc,jpg,png}}", |builder, matched| {
        register_raw_file(
            builder,
            source_root.join(matched.complete()),
            matched.complete(),
        )
    });
    source_tree.route(
        "{{index,contests,about,others/**/*}.html}.mkd",
        |builder, matched| {
            register_markdown_page(
                builder,
                "general",
                source_root.join(matched.complete()),
                matched.get(1).unwrap(),
                None,
            )
        },
    );
    source_tree.route("{**/*.html}.hbs", |builder, matched| {
        register_direct_handlebar_page(
            builder,
            source_root.join(matched.complete()),
            matched.get(1).unwrap(),
        )
    });
    source_tree.route("{aoj/*.html}.mkd", |builder, matched| {
        register_markdown_page(
            builder,
            "problem",
            source_root.join(matched.complete()),
            matched.get(1).unwrap(),
            Some("AOJ {{problem.id}} {{problem.name}}"),
        )
    });
    source_tree.route("{atcoder/**/*.html}.mkd", |builder, matched| {
        register_markdown_page(
            builder,
            "problem",
            source_root.join(matched.complete()),
            matched.get(1).unwrap(),
            Some("{{problem.contest}} {{problem.id}} {{problem.name}}"),
        )
    });
    source_tree.route("{topcoder/*/index.html}.mkd", |builder, matched| {
        register_markdown_page(
            builder,
            "topcoder_contest",
            source_root.join(matched.complete()),
            matched.get(1).unwrap(),
            Some("TopCoder {{contest.round}}"),
        )
    });
    source_tree.route("{topcoder/*/[A-Z]*.html}.mkd", |builder, matched| {
        register_markdown_page(
            builder,
            "problem",
            source_root.join(matched.complete()),
            matched.get(1).unwrap(),
            Some("TopCoder {{problem.round}} {{join problem.level}} {{problem.name}}"),
        )
    });
    source_tree.route("{yukicoder/*.html}.mkd", |builder, matched| {
        register_markdown_page(
            builder,
            "problem",
            source_root.join(matched.complete()),
            matched.get(1).unwrap(),
            Some("yukicoder {{problem.id}} {{problem.name}}"),
        )
    });

    source_tree.complete(&mut builder)?;
    register_tag_pages(&mut builder)?;
    builder.compile(args.target)?;
    Ok(())
}
