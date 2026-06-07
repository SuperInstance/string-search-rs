//! Z-algorithm based string matching.
//!
//! The Z-function for a string `s` computes `z[i]` = the length of the longest
//! substring starting at `i` that matches a prefix of `s`. For pattern matching,
//! we concatenate `pattern + "$" + text` and look for Z-values equal to the pattern length.
//!
//! Time complexity: O(n + m).

use crate::Matches;

/// Compute the Z-function of a string.
///
/// Returns a vector where `z[i]` is the length of the longest substring starting
/// at position `i` that matches a prefix of the input string. By convention, `z[0] = n`.
///
/// # Examples
///
/// ```
/// use string_search_rs::z_algorithm;
///
/// let z = z_algorithm::z_function("aabxaab");
/// assert_eq!(z[0], 7); // entire string matches itself
/// assert_eq!(z[4], 3); // "aab" matches prefix "aab"
/// ```
pub fn z_function(s: &str) -> Vec<usize> {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut z = vec![0usize; n];
    if n == 0 {
        return z;
    }

    z[0] = n;
    let mut l = 0usize;
    let mut r = 0usize;

    for i in 1..n {
        if i < r {
            z[i] = std::cmp::min(r - i, z[i - l]);
        }
        while i + z[i] < n && bytes[z[i]] == bytes[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }

    z
}

/// Search for all occurrences of `pattern` in `text` using the Z-algorithm.
///
/// Concatenates `pattern + "$" + text`, computes the Z-function, and reports
/// positions where `z[i] == pattern.len()`.
///
/// The `$` separator character is chosen to not appear in typical text.
/// If the pattern or text contains `\x00`, it still works correctly because
/// the separator is always at position `m` in the concatenated string.
///
/// # Examples
///
/// ```
/// use string_search_rs::z_algorithm;
///
/// let matches = z_algorithm::find("aabxaabcaabxaa", "aab");
/// assert_eq!(matches, vec![0, 4, 8]);
/// ```
pub fn find(text: &str, pattern: &str) -> Matches {
    let n = text.len();
    let m = pattern.len();

    if m == 0 {
        return (0..=n).collect();
    }

    // Concatenate pattern + "$" + text using \x00 as separator
    let combined: Vec<u8> = pattern
        .as_bytes()
        .iter()
        .copied()
        .chain(std::iter::once(0x00)) // null byte separator
        .chain(text.as_bytes().iter().copied())
        .collect();

    let combined_str = unsafe { std::str::from_utf8_unchecked(&combined) };
    let z = z_function(combined_str);

    let mut result = Vec::new();
    // Pattern occupies positions [0..m), separator at m, text starts at m+1
    for (i, &zval) in z.iter().enumerate().skip(m + 1) {
        if zval == m {
            result.push(i - m - 1);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z_function_basic() {
        let z = z_function("aaaa");
        assert_eq!(z, vec![4, 3, 2, 1]);
    }

    #[test]
    fn test_z_function_no_prefix_match() {
        let z = z_function("abcd");
        assert_eq!(z, vec![4, 0, 0, 0]);
    }

    #[test]
    fn test_find_basic() {
        assert_eq!(find("abcabc", "abc"), vec![0, 3]);
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
