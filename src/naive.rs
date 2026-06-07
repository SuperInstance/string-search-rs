//! Naive (brute-force) string matching algorithm.
//!
//! Time complexity: O(n·m) in the worst case, where `n` is the text length
//! and `m` is the pattern length. Simple and correct by definition.

use crate::Matches;
use crate::overlap::OverlapMode;

/// Search for all occurrences of `pattern` in `text` using brute-force comparison.
///
/// Returns a sorted vector of zero-based starting indices.
///
/// # Overlapping
///
/// Overlapping matches are reported by default. Use [`find_with_overlap`] to control this.
///
/// # Examples
///
/// ```
/// use string_search_rs::naive;
///
/// let matches = naive::find("abcabc", "abc");
/// assert_eq!(matches, vec![0, 3]);
/// ```
pub fn find(text: &str, pattern: &str) -> Matches {
    find_with_overlap(text, pattern, OverlapMode::Allow)
}

/// Search with explicit overlap control.
///
/// When `mode` is [`OverlapMode::Disallow`], after a match at position `i`,
/// the next search starts at `i + pattern.len()`, avoiding overlapping results.
///
/// # Examples
///
/// ```
/// use string_search_rs::{naive, overlap::OverlapMode};
///
/// let matches = naive::find_with_overlap("aaaa", "aa", OverlapMode::Disallow);
/// assert_eq!(matches, vec![0, 2]);
/// ```
pub fn find_with_overlap(text: &str, pattern: &str, mode: OverlapMode) -> Matches {
    let n = text.len();
    let m = pattern.len();

    if m == 0 {
        // Empty pattern matches at every position (including n)
        return (0..=n).collect();
    }

    let text_bytes = text.as_bytes();
    let pat_bytes = pattern.as_bytes();
    let mut result = Vec::new();
    let mut i = 0;

    while i + m <= n {
        if text_bytes[i..i + m] == pat_bytes[..] {
            result.push(i);
            i += match mode {
                OverlapMode::Allow => 1,
                OverlapMode::Disallow => m,
                OverlapMode::Extend { .. } => m,
            };
        } else {
            i += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(find("hello world", "world"), vec![6]);
    }

    #[test]
    fn test_multiple() {
        assert_eq!(find("ababab", "ab"), vec![0, 2, 4]);
    }

    #[test]
    fn test_empty_pattern_returns_all_positions() {
        assert_eq!(find("abc", ""), vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_empty_text_nonempty_pattern() {
        assert!(find("", "a").is_empty());
    }

    #[test]
    fn test_overlap_disallow() {
        assert_eq!(find_with_overlap("aaaa", "aa", OverlapMode::Disallow), vec![0, 2]);
    }
}
