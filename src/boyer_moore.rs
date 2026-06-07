//! Boyer-Moore string matching algorithm (bad character heuristic).
//!
//! Achieves sub-linear performance in practice by skipping sections of the text
//! that cannot possibly match. Uses the bad character rule to determine the skip distance.
//!
//! Time complexity: O(n/m) best case, O(n + m) worst case.

use crate::Matches;

/// Build the bad character table.
///
/// For each byte value (0–255), stores the rightmost occurrence in the pattern.
/// If a byte does not appear, the entry is `-1i32` (indicating it can skip past).
///
/// # Examples
///
/// ```
/// use string_search_rs::boyer_moore;
///
/// let table = boyer_moore::build_bad_char_table("abc");
/// assert!(table.len() == 256);
/// ```
pub fn build_bad_char_table(pattern: &str) -> Vec<i32> {
    let m = pattern.len();
    let mut table = vec![-1i32; 256];
    let pat_bytes = pattern.as_bytes();

    for (i, &b) in pat_bytes.iter().enumerate() {
        table[b as usize] = i as i32;
    }

    let _ = m; // suppress unused warning
    table
}

/// Search for all occurrences of `pattern` in `text` using the Boyer-Moore algorithm
/// with the bad character heuristic.
///
/// Returns a sorted vector of zero-based starting indices.
///
/// The algorithm scans the pattern from right to left. When a mismatch occurs,
/// the bad character table determines how far to shift the pattern, often skipping
/// large portions of the text.
///
/// # Examples
///
/// ```
/// use string_search_rs::boyer_moore;
///
/// let matches = boyer_moore::find("ABAAABCDABC", "ABC");
/// assert_eq!(matches, vec![4, 8]);
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

    let bad_char = build_bad_char_table(pattern);
    let text_bytes = text.as_bytes();
    let pat_bytes = pattern.as_bytes();

    let mut result = Vec::new();
    let mut s = 0usize; // shift of pattern w.r.t. text

    while s <= n - m {
        let mut j = m as i32 - 1;

        // Reduce j while characters match
        while j >= 0 && pat_bytes[j as usize] == text_bytes[s + j as usize] {
            j -= 1;
        }

        if j < 0 {
            // Pattern found at position s
            result.push(s);

            // Shift by full pattern length or 1
            let next_pos = s + m;
            if next_pos < n {
                let shift = (m as i32 - bad_char[text_bytes[next_pos] as usize] - 1).max(1) as usize;
                s += shift;
            } else {
                s += 1;
            }
        } else {
            // Mismatch: shift by bad character rule
            let bc_shift = (j - bad_char[text_bytes[s + j as usize] as usize]).max(1);
            s += bc_shift as usize;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bad_char_table_values() {
        let table = build_bad_char_table("abc");
        assert_eq!(table[b'a' as usize], 0);
        assert_eq!(table[b'b' as usize], 1);
        assert_eq!(table[b'c' as usize], 2);
        assert_eq!(table[b'z' as usize], -1);
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

    #[test]
    fn test_find_single_char() {
        assert_eq!(find("abcba", "b"), vec![1, 3]);
    }
}
