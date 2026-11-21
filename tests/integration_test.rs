//! Integration tests for sigmaos-archwiki.

use archwiki::{Namespace, PageTitle, parse_links, Revision, RevisionHistory, User, wiki_url};

#[test]
fn test_wiki_url_generation() {
    let t = PageTitle::parse("Installation guide").unwrap();
    assert_eq!(wiki_url(&t), "https://wiki.archlinux.org/title/Installation_guide");
}

#[test]
fn test_namespaced_title_roundtrip() {
    let t = PageTitle::parse("Arch Linux:Packages").unwrap();
    assert_eq!(t.namespace, Namespace::ArchLinux);
    assert_eq!(t.display(), "Arch Linux:Packages");
}

#[test]
fn test_wikitext_link_extraction() {
    let wikitext = "The [[Main Page]] links to [[Category:Installation]] and [[File:logo.png|logo]].";
    let links = parse_links(wikitext);
    assert!(links.iter().any(|l| l.target == "Main Page"));
    assert!(links.iter().any(|l| l.target == "Category:Installation"));
    assert!(links.iter().any(|l| l.target == "File:logo.png"));
}

#[test]
fn test_revision_history() {
    let mut h = RevisionHistory::default();
    for (id, content) in [(1, "v1"), (2, "v2"), (3, "v3")] {
        h.revisions.push(Revision {
            id, title: "P".into(), content: content.into(),
            user: User { id: 1, username: "editor".into() },
            timestamp: 0, comment: String::new(), minor: false, bot: false,
        });
    }
    let (old, new) = h.diff(1, 3).unwrap();
    assert_eq!(old.content, "v1");
    assert_eq!(new.content, "v3");
}
