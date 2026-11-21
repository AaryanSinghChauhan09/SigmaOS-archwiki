//! Page title + namespace model — MediaWiki semantics.

use crate::WikiError;

/// MediaWiki namespace ids (subset used by Arch wiki).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Namespace {
    /// Main article space.
    Main = 0,
    /// User pages.
    User = 2,
    /// User talk.
    UserTalk = 3,
    /// Arch Linux namespace.
    ArchLinux = 100,
    /// Arch Linux talk.
    ArchLinuxTalk = 101,
    /// Help pages.
    Help = 12,
    /// Category pages.
    Category = 14,
    /// Template pages.
    Template = 10,
    /// File pages.
    File = 6,
}

impl Namespace {
    /// Parse namespace id.
    pub fn from_id(id: u32) -> Option<Self> {
        Some(match id {
            0 => Self::Main,
            2 => Self::User,
            3 => Self::UserTalk,
            6 => Self::File,
            10 => Self::Template,
            12 => Self::Help,
            14 => Self::Category,
            100 => Self::ArchLinux,
            101 => Self::ArchLinuxTalk,
            _ => return None,
        })
    }

    /// Canonical namespace prefix (`Category:`, `User:`, etc.).
    pub fn prefix(&self) -> &'static str {
        match self {
            Self::Main => "",
            Self::User => "User:",
            Self::UserTalk => "User talk:",
            Self::ArchLinux => "Arch Linux:",
            Self::ArchLinuxTalk => "Arch Linux talk:",
            Self::Help => "Help:",
            Self::Category => "Category:",
            Self::Template => "Template:",
            Self::File => "File:",
        }
    }

    /// Whether this namespace is a talk page.
    pub fn is_talk(&self) -> bool {
        matches!(self, Self::UserTalk | Self::ArchLinuxTalk)
    }
}

/// A fully-qualified page title.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PageTitle {
    /// Namespace.
    pub namespace: Namespace,
    /// Page name within namespace (no prefix).
    pub name: String,
}

impl PageTitle {
    /// Parse `Title` or `Namespace:Title` text.
    pub fn parse(text: &str) -> Result<Self, WikiError> {
        let text = text.trim();
        if text.is_empty() {
            return Err(WikiError::InvalidTitle("empty".into()));
        }
        // Check for namespace prefix.
        for ns in [
            Namespace::Category,
            Namespace::Template,
            Namespace::User,
            Namespace::UserTalk,
            Namespace::ArchLinux,
            Namespace::ArchLinuxTalk,
            Namespace::Help,
            Namespace::File,
        ] {
            let p = ns.prefix();
            if !p.is_empty() && text.to_lowercase().starts_with(&p.to_lowercase()) {
                let name = text[p.len()..].trim().to_string();
                if name.is_empty() {
                    return Err(WikiError::InvalidTitle(format!("empty after {p}")));
                }
                return Ok(Self { namespace: ns, name });
            }
        }
        // Interwiki-style or plain main title.
        Ok(Self { namespace: Namespace::Main, name: text.to_string() })
    }

    /// Canonical display form (`Category:Foo` or `Foo`).
    pub fn display(&self) -> String {
        format!("{}{}", self.namespace.prefix(), self.name)
    }

    /// URL path (`/title/Category:Foo`).
    pub fn url_path(&self) -> String {
        format!("/title/{}", url_encode(&self.display()))
    }

    /// Whether title is in main namespace.
    pub fn is_main(&self) -> bool {
        self.namespace == Namespace::Main
    }
}

/// A wiki category membership.
#[derive(Debug, Clone)]
pub struct Category {
    /// Category title.
    pub title: PageTitle,
    /// Sort key override (optional).
    pub sort_key: Option<String>,
}

/// A wiki page.
#[derive(Debug, Clone)]
pub struct Page {
    /// Page title.
    pub title: PageTitle,
    /// Latest revision id.
    pub latest_revision: u64,
    /// Categories this page belongs to.
    pub categories: Vec<Category>,
}

/// MediaWiki-style URL for a title.
pub fn wiki_url(title: &PageTitle) -> String {
    format!("https://wiki.archlinux.org{}", title.url_path())
}

/// Minimal percent-encoding for wiki titles (spaces → _, reserved chars).
fn url_encode(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        match c {
            ' ' => out.push('_'),
            '/' | '?' | '#' | '[' | ']' | '%' | '&' | '=' | '+' => {
                out.push_str(&format!("%{:02X}", c as u32))
            }
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_namespaces() {
        let t = PageTitle::parse("Category:Installation").unwrap();
        assert_eq!(t.namespace, Namespace::Category);
        assert_eq!(t.name, "Installation");
        let t = PageTitle::parse("Installation").unwrap();
        assert_eq!(t.namespace, Namespace::Main);
    }

    #[test]
    fn test_url_path() {
        let t = PageTitle::parse("Category:Arch install").unwrap();
        assert_eq!(t.url_path(), "/title/Category:Arch_install");
    }

    #[test]
    fn test_talk_namespace() {
        let t = PageTitle::parse("Arch Linux talk:Main Page").unwrap();
        assert!(t.namespace.is_talk());
    }
}
