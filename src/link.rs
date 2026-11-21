//! Wiki link parsing — `[[link]]`, `[[link|text]]`, categories, files.

use crate::page::{Namespace, PageTitle};
use crate::WikiError;

/// Kind of wiki link.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkKind {
    /// Regular article link `[[Target]]`.
    Article,
    /// Link with display text `[[Target|Text]]`.
    ArticleText,
    /// Category link `[[Category:X]]`.
    Category,
    /// File/image link `[[File:X]]`.
    File,
    /// External URL `https://...`.
    External,
}

/// A parsed wiki link.
#[derive(Debug, Clone)]
pub struct Link {
    /// Target title or URL.
    pub target: String,
    /// Display text (for `[[target|text]]`).
    pub text: Option<String>,
    /// Link kind.
    pub kind: LinkKind,
}

/// Extract all `[[...]]` links and bare URLs from wikitext.
pub fn parse_links(wikitext: &str) -> Vec<Link> {
    let mut links = Vec::new();
    let mut rest = wikitext;
    while let Some(start) = rest.find("[[") {
        let after = &rest[start + 2..];
        if let Some(end) = after.find("]]") {
            let inner = &after[..end];
            links.push(parse_wiki_link(inner));
            rest = &after[end + 2..];
        } else {
            break;
        }
    }
    // Bare external URLs.
    for word in wikitext.split_whitespace() {
        if word.starts_with("http://") || word.starts_with("https://") {
            links.push(Link {
                target: word.trim_matches(|c| c == ')' || c == ']' || c == '.').to_string(),
                text: None,
                kind: LinkKind::External,
            });
        }
    }
    links
}

fn parse_wiki_link(inner: &str) -> Link {
    let (target, text) = match inner.split_once('|') {
        Some((t, x)) => (t.trim(), Some(x.trim().to_string())),
        None => (inner.trim(), None),
    };
    let kind = if target.to_lowercase().starts_with("category:") {
        LinkKind::Category
    } else if target.to_lowercase().starts_with("file:") {
        LinkKind::File
    } else if text.is_some() {
        LinkKind::ArticleText
    } else {
        LinkKind::Article
    };
    Link { target: target.to_string(), text, kind }
}

/// Resolve a link target to a [`PageTitle`], handling namespaces.
pub fn resolve_title(link: &Link) -> Result<PageTitle, WikiError> {
    PageTitle::parse(&link.target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_links() {
        let text = "See [[Installation guide|the guide]] and [[Category:Basics]]. Also https://example.com";
        let links = parse_links(text);
        assert!(links.iter().any(|l| l.kind == LinkKind::ArticleText));
        assert!(links.iter().any(|l| l.kind == LinkKind::Category));
        assert!(links.iter().any(|l| l.kind == LinkKind::External));
    }

    #[test]
    fn test_category_detection() {
        let links = parse_links("[[Category:Arch Linux]]");
        assert_eq!(links[0].kind, LinkKind::Category);
    }

    #[test]
    fn test_resolve_namespaced() {
        let l = parse_links("[[User:Alice]]")[0].clone();
        let t = resolve_title(&l).unwrap();
        assert_eq!(t.namespace, Namespace::User);
    }
}
