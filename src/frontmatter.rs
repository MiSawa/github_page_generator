use anyhow::Result;
use lazy_regex::regex_find;

use crate::Value;

pub fn split_and_parse_frontmatter(s: &str) -> Result<(Option<Value>, &str)> {
    let (frontmatter, rest) = split_frontmatter(s);
    Ok((frontmatter.map(serde_yaml::from_str).transpose()?, rest))
}

fn split_frontmatter(s: &str) -> (Option<&str>, &str) {
    if let Some(s) = s.strip_prefix("---").map(|s| s.trim_start_matches('-')) {
        if let Some(s) = s.strip_prefix(|c| c == '\r' || c == '\n') {
            if let Some(front_matter) = regex_find!(r#".*?^-{3,}$"#sm, s) {
                let rest = s[front_matter.len()..].trim_start();
                let front_matter = front_matter.trim_end_matches('-').trim_end();
                return (Some(front_matter), rest);
            }
        }
    }
    (None, s)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_basic() {
        let (front, rest) = split_frontmatter("---\nfoo: 1\nbar: 2\n---\nfoo");
        assert_eq!(front, Some("foo: 1\nbar: 2"));
        assert_eq!(rest, "foo");
    }

    #[test]
    fn test_multiple_delimiters() {
        let (front, rest) = split_frontmatter("---\nfoo: 1\nbar: 2\n---\nfoo\n---\nbar");
        assert_eq!(front, Some("foo: 1\nbar: 2"));
        assert_eq!(rest, "foo\n---\nbar");
    }

    #[test]
    fn test_more_dashes() {
        let (front, rest) = split_frontmatter("-----\nfoo: 1\nbar: 2\n----\nfoo\n---\nbar");
        assert_eq!(front, Some("foo: 1\nbar: 2"));
        assert_eq!(rest, "foo\n---\nbar");
    }
}
