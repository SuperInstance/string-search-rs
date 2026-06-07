//! Suffix array based string matching.
//!
//! Constructs a suffix array of the text and uses binary search to find all
//! occurrences of a pattern. This approach is efficient for multiple queries
//! on the same text.
//!
//! Suffix array construction: O(n² log n) using the naive sort approach.
//! Search: O(m log n) per query.

use crate::Matches;

/// Build a suffix array for the given text.
///
/// Returns a vector of starting indices (byte offsets) sorted lexicographically
/// by the corresponding suffix. Works correctly with UTF-8 text.
///
/// Uses the straightforward O(n² log n) approach: sort pairs of (suffix, index).
/// Suitable for research and educational purposes.
///
/// # Examples
///
/// ```
/// use string_search_rs::suffix_search;
///
/// let sa = suffix_search::build_suffix_array("banana");
/// assert_eq!(sa, vec![5, 3, 1, 0, 4, 2]);
/// ```
pub fn build_suffix_array(text: &str) -> Vec<usize> {
    let bytes = text.as_bytes();
    let n = bytes.len();
    let mut suffixes: Vec<usize> = (0..n).collect();

    suffixes.sort_by(|&a, &b| bytes[a..].cmp(&bytes[b..]));

    suffixes
}

/// Search for a pattern in a suffix array using binary search.
///
/// Finds the leftmost and rightmost suffixes that begin with `pattern`,
/// returning all matching starting positions (byte offsets).
///
/// # Examples
///
/// ```
/// use string_search_rs::suffix_search;
///
/// let text = "banana";
/// let sa = suffix_search::build_suffix_array(text);
/// let found = suffix_search::search_suffix_array(text, &sa, "ana");
/// assert_eq!(found, vec![1, 3]);
/// ```
pub fn search_suffix_array(text: &str, sa: &[usize], pattern: &str) -> Vec<usize> {
    let m = pattern.len();
    if m == 0 || text.is_empty() {
        return Vec::new();
    }

    let text_bytes = text.as_bytes();
    let pat_bytes = pattern.as_bytes();
    let n = sa.len();

    // Find leftmost occurrence (first suffix >= pattern)
    let mut lo = 0isize;
    let mut hi = n as isize - 1;
    let mut left = n;

    while lo <= hi {
        let mid = (lo + hi) / 2;
        let suffix_start = sa[mid as usize];
        let suffix = &text_bytes[suffix_start..];
        let suffix_prefix = if suffix.len() >= m {
            &suffix[..m]
        } else {
            suffix
        };

        if suffix_prefix < pat_bytes {
            lo = mid + 1;
        } else {
            if suffix_prefix == pat_bytes {
                left = mid as usize;
            }
            hi = mid - 1;
        }
    }

    if left == n {
        return Vec::new();
    }

    // Find rightmost occurrence
    let mut lo = 0isize;
    let mut hi = n as isize - 1;
    let mut right = 0usize;

    while lo <= hi {
        let mid = (lo + hi) / 2;
        let suffix_start = sa[mid as usize];
        let suffix = &text_bytes[suffix_start..];
        let suffix_prefix = if suffix.len() >= m {
            &suffix[..m]
        } else {
            suffix
        };

        if suffix_prefix > pat_bytes {
            hi = mid - 1;
        } else {
            if suffix_prefix == pat_bytes {
                right = mid as usize;
            }
            lo = mid + 1;
        }
    }

    let mut result: Vec<usize> = sa[left..=right].to_vec();
    result.sort();
    result
}

/// Convenience function: search for all occurrences of `pattern` in `text`
/// using suffix array construction and binary search.
///
/// Returns a sorted vector of zero-based starting byte offsets.
///
/// # Examples
///
/// ```
/// use string_search_rs::suffix_search;
///
/// let matches = suffix_search::find("banana", "ana");
/// assert_eq!(matches, vec![1, 3]);
/// ```
pub fn find(text: &str, pattern: &str) -> Matches {
    let n = text.len();
    let m = pattern.len();

    if m == 0 {
        return (0..=n).collect();
    }

    if m > n {
        return Vec::new();
    }

    let sa = build_suffix_array(text);
    search_suffix_array(text, &sa, pattern)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_array_banana() {
        let sa = build_suffix_array("banana");
        assert_eq!(sa, vec![5, 3, 1, 0, 4, 2]);
    }

    #[test]
    fn test_suffix_array_empty() {
        let sa = build_suffix_array("");
        assert!(sa.is_empty());
    }

    #[test]
    fn test_suffix_array_single_char() {
        let sa = build_suffix_array("a");
        assert_eq!(sa, vec![0]);
    }

    #[test]
    fn test_search_found() {
        let text = "banana";
        let sa = build_suffix_array(text);
        let found = search_suffix_array(text, &sa, "ana");
        assert_eq!(found, vec![1, 3]);
    }

    #[test]
    fn test_search_not_found() {
        let text = "banana";
        let sa = build_suffix_array(text);
        let found = search_suffix_array(text, &sa, "xyz");
        assert!(found.is_empty());
    }

    #[test]
    fn test_find_basic() {
        assert_eq!(find("abcabc", "abc"), vec![0, 3]);
    }
}
