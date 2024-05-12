use std::{collections::HashMap, ops::Range};

use anyhow::{anyhow, ensure, Result};
use camino::Utf8Path as Path;
use either::Either;
use itertools::Itertools as _;
use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, InlineStr, Options, Parser, Tag, TagEnd};
use serde_json::Value;

use crate::{frontmatter, highlight::source_code_to_html};

pub struct MarkdownDocument {
    pub frontmatter: Option<Value>,
    pub content: String,
}

impl MarkdownDocument {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let content = std::fs::read_to_string(path.as_ref())?;
        let (frontmatter, content) = frontmatter::split_and_parse_frontmatter(&content)?;
        Ok(Self {
            frontmatter,
            content: content.to_owned(),
        })
    }

    pub fn render_html(&self) -> Result<String> {
        let events = parse_to_events(&self.content)?;
        let mut s = String::new();
        html::push_html(&mut s, events.into_iter());
        Ok(s)
    }
}

fn parse_to_events(markdown: &str) -> Result<Vec<Event>> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_MATH);
    options.insert(Options::ENABLE_GFM);

    let mut parsed_events = vec![];
    let mut syntax_highlighter = SyntaxHighlighter::default();
    let mut footnote_handler = FootnoteHandler::default();
    let mut container_handler = ContainerHandler::default();
    for event in Parser::new_ext(markdown, options) {
        let event = fix_math(event);
        let Some(event) = syntax_highlighter.handle(event)? else {
            continue;
        };
        let Some(event) = footnote_handler.handle(event)? else {
            continue;
        };
        parsed_events.extend(container_handler.handle(event)?);
    }
    parsed_events.extend(footnote_handler.collect_rest()?);
    Ok(parsed_events)
}

#[derive(Default)]
struct ContainerHandler<'a> {
    close_events: Vec<Event<'a>>,
    depth_in_summary: Option<usize>,
}
impl<'a> ContainerHandler<'a> {
    fn handle(&mut self, event: Event<'a>) -> Result<impl IntoIterator<Item = Event<'a>>> {
        match &event {
            Event::Start(_) => {
                if let Some(d) = self.depth_in_summary.as_mut() {
                    *d += 1;
                }
            }
            Event::End(_) => {
                if let Some(d) = self.depth_in_summary.as_mut() {
                    if *d == 0 {
                        self.depth_in_summary = None;
                        let close_summary = self
                            .close_events
                            .pop()
                            .ok_or_else(|| anyhow!("Close tag for summary not found"))?;
                        return Ok(Either::Right(vec![close_summary, event].into_iter()));
                    } else {
                        *d -= 1;
                    }
                }
            }
            Event::Text(text) => {
                if let Some(container) = text.strip_prefix(":::").map(str::trim_start) {
                    let mut events = vec![];
                    if let Some(rest) = container.strip_prefix("indent") {
                        events.push(Event::Html(CowStr::Borrowed(r#"<div class="indent">"#)));
                        events.push(Event::Text(rest.to_owned().into()));
                        self.close_events
                            .push(Event::Html(CowStr::Borrowed(r#"</div>"#)));
                    } else if let Some(summary) = container.strip_prefix("details") {
                        events.push(Event::Html(CowStr::Borrowed(r#"<details><summary>"#)));
                        events.push(Event::Text(summary.to_owned().into()));
                        self.close_events
                            .push(Event::Html(CowStr::Borrowed("</details>")));
                        self.close_events
                            .push(Event::Html(CowStr::Borrowed("</summary>")));
                        self.depth_in_summary = Some(0);
                    } else {
                        ensure!(
                            container.is_empty(),
                            "What does container tag {container} even mean?"
                        );
                        let event = self
                            .close_events
                            .pop()
                            .ok_or_else(|| anyhow!("Too many close container tag"))?;
                        events.push(event);
                    }
                    return Ok(Either::Right(events.into_iter()));
                }
            }
            _ => {}
        }
        Ok(Either::Left(std::iter::once(event)))
    }
}

fn fix_math(event: Event) -> Event {
    // TODO
    #[allow(unused)]
    fn cow_substring(cow: CowStr, range: Range<usize>) -> CowStr {
        match cow {
            CowStr::Boxed(s) => s[range.start..range.end].to_owned().into(),
            CowStr::Borrowed(s) => CowStr::Borrowed(&s[range.start..range.end]),
            CowStr::Inlined(s) => {
                CowStr::Inlined(InlineStr::try_from(&s[range.start..range.end]).unwrap())
            }
        }
    }
    match event {
        Event::Code(code) if code.len() >= 2 && code.starts_with('$') && code.ends_with('$') => {
            if code.len() >= 4 && code.starts_with("$$") && code.ends_with("$$") {
                //let range = 2..code.len() - 2;
                // Event::DisplayMath(substring(code, range))
                Event::Html(code)
            } else {
                // let range = 1..code.len() - 1;
                // Event::InlineMath(substring(code, range))
                Event::InlineHtml(code)
            }
        }
        Event::InlineMath(math)
            if math.len() >= 2 && math.starts_with('`') && math.ends_with('`') =>
        {
            let prefix_ticks = math.chars().take_while(|&c| c == '`').count();
            let suffix_ticks = math.chars().rev().take_while(|&c| c == '`').count();
            let ticks = prefix_ticks.min(suffix_ticks);
            if ticks == math.len() {
                //Event::InlineMath(math)
                Event::InlineHtml(format!("${math}$").into())
            } else {
                let range = ticks..math.len() - ticks;
                // Event::InlineMath(cow_substring(math, range))
                Event::InlineHtml(format!("${}$", &math[range]).into())
            }
        }
        Event::InlineMath(math) => Event::InlineHtml(format!("${math}$").into()),
        Event::DisplayMath(math)
            if math.len() >= 2 && math.starts_with('`') && math.ends_with('`') =>
        {
            let prefix_ticks = math.chars().take_while(|&c| c == '`').count();
            let suffix_ticks = math.chars().rev().take_while(|&c| c == '`').count();
            let ticks = prefix_ticks.min(suffix_ticks);
            if ticks == math.len() {
                //Event::DisplayMath(math)
                Event::Html(format!("$${math}$$").into())
            } else {
                let range = ticks..math.len() - ticks;
                // Event::DisplayMath(cow_substring(math, range))
                Event::Html(format!("$${}$$", &math[range]).into())
            }
        }
        Event::DisplayMath(math) => Event::Html(format!("$${math}$$").into()),
        _ => event,
    }
}

#[derive(Default)]
struct SyntaxHighlighter {
    in_code_block: bool,
    lang: Option<String>,
}
impl SyntaxHighlighter {
    fn handle<'a>(&mut self, event: Event<'a>) -> Result<Option<Event<'a>>> {
        Ok(match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref lang))) => {
                self.in_code_block = true;
                if !lang.is_empty() {
                    self.lang = Some(lang.to_string());
                }
                None
            }
            Event::Text(ref text) => Some(if self.in_code_block {
                let html = source_code_to_html(text.as_ref(), self.lang.as_deref())?;
                Event::Html(html.into())
            } else {
                event
            }),
            Event::End(TagEnd::CodeBlock) => {
                if self.in_code_block {
                    self.in_code_block = false;
                    self.lang = None;
                    None
                } else {
                    Some(event)
                }
            }
            _ => Some(event),
        })
    }
}

#[derive(Default)]
struct FootnoteHandler<'a> {
    notes: HashMap<String, Vec<Event<'a>>>,
    footnote_stack: Vec<(String, Vec<Event<'a>>)>,
    referenced_name_to_id_refcount: HashMap<String, (usize, usize)>,
}
impl<'a> FootnoteHandler<'a> {
    fn handle(&mut self, event: Event<'a>) -> Result<Option<Event<'a>>> {
        fn escape_name(name: &str) -> String {
            let mut s = String::new();
            pulldown_cmark_escape::escape_html(&mut s, name).unwrap();
            s
        }
        Ok(match event {
            Event::Start(Tag::FootnoteDefinition(ref name)) => {
                let name = escape_name(name);
                self.footnote_stack.push((name.to_string(), vec![]));
                None
            }
            Event::End(TagEnd::FootnoteDefinition) => {
                let (name, events) = self.footnote_stack.pop().unwrap();
                let existed = self.notes.insert(name.clone(), events);
                ensure!(existed.is_none(), "Footnote {name} defined multiple times");
                None
            }
            Event::FootnoteReference(name) => {
                let name = escape_name(&name);
                let next_id = 1 + self.referenced_name_to_id_refcount.len();
                let (id, refcount) = self
                    .referenced_name_to_id_refcount
                    .entry(name.to_string())
                    .or_insert((next_id, 0));
                *refcount += 1;
                let html = Event::InlineHtml(format!(r##"<sup class="footnote-reference" id="fnref:{name}-{refcount}"><a href="#fn:{name}" class="footnote">{id}</a></sup>"##).into());
                if let Some(notes) = self.footnote_stack.last_mut() {
                    notes.1.push(html);
                    None
                } else {
                    Some(html)
                }
            }
            _ if !self.footnote_stack.is_empty() => {
                self.footnote_stack.last_mut().unwrap().1.push(event);
                None
            }
            _ => Some(event),
        })
    }
    fn collect_rest(mut self) -> Result<Vec<Event<'a>>> {
        assert!(self.footnote_stack.is_empty());
        if self.referenced_name_to_id_refcount.is_empty() {
            return Ok(vec![]);
        }
        let mut rest = vec![
            // Event::Rule,
            Event::Html(r#"<div class="footnotes"><ol>"#.into()),
        ];
        for (name, (_, refcount)) in self
            .referenced_name_to_id_refcount
            .into_iter()
            .sorted_by_key(|&(_, (id, _))| id)
        {
            let mut note = self
                .notes
                .remove(&name)
                .ok_or_else(|| anyhow!("Footnote {name} doesn't have definition"))?;
            rest.push(Event::InlineHtml(format!(r#"<li id="fn:{name}">"#).into()));
            let mut last = None;
            if note.last() == Some(&Event::End(TagEnd::Paragraph)) {
                last = Some(note.pop().unwrap());
            }
            rest.extend(note);
            for i in 1..=refcount {
                if i == 1 {
                    rest.push(Event::Html(
                        format!(r##" <a href="#fnref:{name}-{i}">↩</a>"##).into(),
                    ));
                } else {
                    rest.push(Event::Html(
                        format!(r##" <a href="#fnref:{name}-{i}">↩<sup>{i}</sup></a>"##).into(),
                    ));
                }
            }
            rest.extend(last);
            rest.push(Event::Html(r##"</li>"##.into()));
        }
        rest.extend([Event::Html(r#"</ol></div>"#.into())]);
        Ok(rest)
    }
}
