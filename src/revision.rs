//! Revision model — MediaWiki revision history.

/// A wiki editor.
#[derive(Debug, Clone)]
pub struct User {
    /// User id.
    pub id: u64,
    /// Username.
    pub username: String,
}

/// A page revision.
#[derive(Debug, Clone)]
pub struct Revision {
    /// Revision id.
    pub id: u64,
    /// Page title (denormalized).
    pub title: String,
    /// Revision content (wikitext).
    pub content: String,
    /// Editor.
    pub user: User,
    /// Edit timestamp (unix).
    pub timestamp: i64,
    /// Edit summary/comment.
    pub comment: String,
    /// Whether this revision was a minor edit.
    pub minor: bool,
    /// Whether this revision was a bot edit.
    pub bot: bool,
}

/// Revision history for a page (ordered newest-first).
#[derive(Debug, Default)]
pub struct RevisionHistory {
    /// Revisions.
    pub revisions: Vec<Revision>,
}

impl RevisionHistory {
    /// Latest revision.
    pub fn latest(&self) -> Option<&Revision> {
        self.revisions.first()
    }

    /// Find revision by id.
    pub fn get(&self, id: u64) -> Option<&Revision> {
        self.revisions.iter().find(|r| r.id == id)
    }

    /// Count revisions by a user.
    pub fn by_user(&self, username: &str) -> usize {
        self.revisions.iter().filter(|r| r.user.username == username).count()
    }

    /// Diff two revisions by id → (old, new) content pair.
    pub fn diff(&self, old_id: u64, new_id: u64) -> Option<(&Revision, &Revision)> {
        let old = self.get(old_id)?;
        let new = self.get(new_id)?;
        Some((old, new))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rev(id: u64, user: &str, content: &str) -> Revision {
        Revision {
            id, title: "Test".into(), content: content.into(),
            user: User { id: 1, username: user.into() },
            timestamp: id as i64, comment: String::new(),
            minor: false, bot: false,
        }
    }

    #[test]
    fn test_history_order() {
        let mut h = RevisionHistory::default();
        h.revisions.push(rev(3, "a", "v3"));
        h.revisions.push(rev(2, "b", "v2"));
        h.revisions.push(rev(1, "a", "v1"));
        assert_eq!(h.latest().unwrap().id, 3);
        assert_eq!(h.by_user("a"), 2);
        let (o, n) = h.diff(1, 3).unwrap();
        assert_eq!(o.content, "v1");
        assert_eq!(n.content, "v3");
    }
}
