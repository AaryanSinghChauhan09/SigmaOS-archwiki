//! SigmaOS archwiki component.
//!
//! Rust implementation of `archlinux/archwiki`: the MediaWiki instance
//! powering wiki.archlinux.org.
//!
//! Upstream is MediaWiki (PHP); this port implements the wiki domain
//! model needed for tooling: page titles/namespaces, revisions, links,
//! categories, and the Arch wiki URL scheme (`/title/X`, `/index.php?title=X`).

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod error;
pub mod link;
pub mod page;
pub mod revision;

pub use error::WikiError;
pub use link::{Link, LinkKind, parse_links};
pub use page::{Category, Namespace, Page, PageTitle, wiki_url};
pub use revision::{Revision, RevisionHistory, User, User as WikiUser};
