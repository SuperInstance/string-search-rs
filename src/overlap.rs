//! Overlap control mode for search algorithms.

/// Controls how overlapping matches are handled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlapMode {
    /// Report all matches, including overlapping ones.
    Allow,
    /// After a match at position `i`, skip to `i + pattern_len` before searching again.
    Disallow,
    /// Internal: extend variant carrying the last match start.
    Extend { last_start: usize },
}
