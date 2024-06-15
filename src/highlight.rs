use std::{fmt::Write as _, sync::OnceLock};

use anyhow::Result;
use itertools::Itertools as _;
use syntect::{
    easy::HighlightLines,
    highlighting::{Color, Theme},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

const BG: Color = Color {
    r: 0x49,
    g: 0x48,
    b: 0x3e,
    a: 0x00,
};
static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
static THEME: OnceLock<Theme> = OnceLock::new();

pub fn source_code_to_html(code: &str, lang: Option<&str>) -> Result<String> {
    let ss = SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines);
    let theme = THEME.get_or_init(|| {
        let assets = syntect_assets::assets::HighlightingAssets::from_binary();
        let mut theme = assets.get_theme("DarkNeon").clone();
        theme.settings.background = Some(BG);
        theme
    });

    let code = code.trim_end();
    let sr = lang
        .and_then(|lang| ss.find_syntax_by_token(lang))
        .unwrap_or_else(|| ss.find_syntax_plain_text());
    // let mut theme = ThemeSet::load_defaults().themes["Solarized (dark)"].clone();
    // if let Some(item) = theme.scopes.iter_mut().find(|item| {
    //     let selectors = &item.scope.selectors;
    //     if selectors.len() == 1 {
    //         selectors[0].extract_single_scope() == Some(Scope::from_str("keyword").unwrap())
    //     } else {
    //         false
    //     }
    // }) {
    //     item.style.font_style = Some(FontStyle::BOLD);
    // }

    const ZERO_MARGIN_PADDING_TOP_BOT: &str =
        "padding-top:0;padding-bottom:0;margin-top:0;margin-bottom:0";

    let lines = code.lines().count();
    let mut html = String::new();
    write!(
        &mut html,
        r#"<pre class="highlight" style="margin:0.8em;background-color:#{:02x}{:02x}{:02x};"><code class="highlight"><table class="highlight"><tbody><tr>"#,
        BG.r, BG.g, BG.b,
    )?;
    write!(
        &mut html,
        r#"<td class="lineno" style="padding-left:0.5em;padding-right:0.5em;border-left:1px solid #000;"><pre class="lineno" style="{ZERO_MARGIN_PADDING_TOP_BOT};color:#f8f8f2;">"#
    )?;
    write!(&mut html, "{}", (1..=lines).format("\n"))?;
    write!(
        &mut html,
        r#"</pre></td><td class="code" style="{ZERO_MARGIN_PADDING_TOP_BOT};padding-left:0.5em;padding-right:0.5em;border-left:1px solid #000;">"#
    )?;

    write!(
        &mut html,
        r#"<pre style="{ZERO_MARGIN_PADDING_TOP_BOT};white-space:pre;">"#
    )?;
    let mut highlighter = HighlightLines::new(sr, theme);
    for line in LinesWithEndings::from(code) {
        let regions = highlighter.highlight_line(line, ss)?;
        syntect::html::append_highlighted_html_for_styled_line(
            &regions[..],
            syntect::html::IncludeBackground::IfDifferent(BG),
            &mut html,
        )?;
    }
    write!(
        &mut html,
        r#"</pre></td></tr></tbody></table></code></pre>"#
    )?;
    Ok(html)
}
