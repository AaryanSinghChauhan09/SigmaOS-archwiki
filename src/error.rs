//! Error types for the archwiki component.

/// Errors in wiki domain operations.
#[derive(Debug, thiserror::Error)]
pub enum WikiError {
    /// Invalid page title or namespace.
    #[error("invalid title: {0}")]
    InvalidTitle(String),

    /// Page or revision not found.
    #[error("not found: {0}")]
    NotFound(String),

    /// Revision conflict (edit based on stale revision id).
    #[error("edit conflict: {0}")]
    EditConflict(String),
}
