//! Knuth-Morris-Pratt (KMP) string matching algorithm.
//!
//! Time complexity: O(n + m) where `n` is the text length and `m` is the pattern length.
//! Uses a failure function (longest proper prefix which is also suffix) to avoid
//! redundant comparisons.

use crate::Matches;

/// Compute the LPS (Longest Proper Prefix which is also Suffix) array for KMP.
///
/// `lps[i]` = length of the longest proper prefix of `pattern[0..=i]` that is also a suffix.
///
/// # Examples
///
/// ```
/// use string_search_rs::kmp;
///
/// let lps = kmp::compute_lps("aabaaab");
/// assert_eq!(lps, vec![0, 1, 0, 1, 2, 2, 3]);
/// ```
pub fn compute_lps(pattern: &str) -> Vec<usize> {
    let m = pattern.len();
    if m == 0 {
        return Vec::new();
    }

    let pat_bytes = pattern.as_bytes();
    let mut lps = vec![0usize; m];
    let mut len = 0usize; // length of the previous longest prefix suffix
    let mut i = 1;

    while i < m {
        if pat_bytes[i] == pat_bytes[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else if len != 0 {
            len = lps[len - 1];
        } else {
            lps[i] = 0;
            i += 1;
        }
    }

    lps
}

/// Search for all occurrences of `pattern` in `text` using the KMP algorithm.
///
/// Returns a sorted vector of zero-based starting indices.
///
/// # Examples
///
/// ```
/// use string_search_rs::kmp;
///
/// let matches = kmp::find("ababcabcabababd", "ababd");
/// assert_eq!(matches, vec![10]);
/// ```
pub fn find(text: &str, pattern: &str) -> Matches {
    let n = text.len();
    let m = pattern.len();

    if m == 0 {
        return (0..=n).collect();
    }

    let lps = compute_lps(pattern);
    let text_bytes = text.as_bytes();
    let pat_bytes = pattern.as_bytes();
    let mut result = Vec::new();

    let mut i = 0usize; // index for text
    let mut j = 0usize; // index for pattern

    while i < n {
        if pat_bytes[j] == text_bytes[i] {
            i += 1;
            j += 1;
        }

        if j == m {
            result.push(i - j);
            j = lps[j - 1];
        } else if i < n && pat_bytes[j] != text_bytes[i] {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lps_simple() {
        assert_eq!(compute_lps("abcd"), vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_lps_repeated() {
        assert_eq!(compute_lps("aaaa"), vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_find_basic() {
        assert_eq!(find("abcabcabc", "abc"), vec![0, 3, 6]);
    }

    #[test]
    fn test_find_no_match() {
        assert!(find("abcdef", "xyz").is_empty());
    }

    #[test]
    fn test_find_overlapping() {
        assert_eq!(find("aaa", "aa"), vec![0, 1]);
    }
}
