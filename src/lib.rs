#![doc = include_str!("../README.md")]

//! # string-search-rs
//!
//! A comprehensive collection of string matching algorithms implemented in pure Rust.
//!
//! ## Algorithms
//!
//! - **Naive** — Brute-force O(n·m) matching
//! - **KMP** — Knuth-Morris-Pratt O(n+m) matching with failure function
//! - **Rabin-Karp** — Rolling hash based O(n+m) average matching
//! - **Boyer-Moore** — Bad-character heuristic, sub-linear in practice
//! - **Z-algorithm** — Z-function based O(n+m) matching
//! - **Suffix Search** — Suffix array construction + binary search
//!
//! All algorithms return `Vec<usize>` of zero-based starting positions.
//!
//! ## Example
//!
//! ```
//! use string_search_rs::{naive, kmp, rabin_karp, boyer_moore, z_algorithm, suffix_search};
//!
//! let text = "abracadabra";
//! let pattern = "abra";
//!
//! let r1 = naive::find(text, pattern);
//! let r2 = kmp::find(text, pattern);
//! let r3 = rabin_karp::find(text, pattern);
//! let r4 = boyer_moore::find(text, pattern);
//! let r5 = z_algorithm::find(text, pattern);
//! let r6 = suffix_search::find(text, pattern);
//!
//! assert_eq!(r1, vec![0, 7]);
//! assert_eq!(r1, r2);
//! assert_eq!(r2, r3);
//! assert_eq!(r3, r4);
//! assert_eq!(r4, r5);
//! assert_eq!(r5, r6);
//! ```

pub mod naive;
pub mod kmp;
pub mod rabin_karp;
pub mod boyer_moore;
pub mod z_algorithm;
pub mod suffix_search;

pub mod overlap;

/// Common result type returned by all search algorithms.
pub type Matches = Vec<usize>;

// Re-export the primary find function from each module at crate root for convenience.

/// Naive (brute-force) string matching. See [`naive::find`].
pub fn find_naive(text: &str, pattern: &str) -> Matches {
    naive::find(text, pattern)
}

/// KMP (Knuth-Morris-Pratt) string matching. See [`kmp::find`].
pub fn find_kmp(text: &str, pattern: &str) -> Matches {
    kmp::find(text, pattern)
}

/// Rabin-Karp string matching. See [`rabin_karp::find`].
pub fn find_rabin_karp(text: &str, pattern: &str) -> Matches {
    rabin_karp::find(text, pattern)
}

/// Boyer-Moore string matching. See [`boyer_moore::find`].
pub fn find_boyer_moore(text: &str, pattern: &str) -> Matches {
    boyer_moore::find(text, pattern)
}

/// Z-algorithm string matching. See [`z_algorithm::find`].
pub fn find_z(text: &str, pattern: &str) -> Matches {
    z_algorithm::find(text, pattern)
}

/// Suffix array string matching. See [`suffix_search::find`].
pub fn find_suffix(text: &str, pattern: &str) -> Matches {
    suffix_search::find(text, pattern)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: all six algorithms should agree on the same result.
    fn all_agree(text: &str, pattern: &str) -> Matches {
        let r1 = naive::find(text, pattern);
        let r2 = kmp::find(text, pattern);
        let r3 = rabin_karp::find(text, pattern);
        let r4 = boyer_moore::find(text, pattern);
        let r5 = z_algorithm::find(text, pattern);
        let r6 = suffix_search::find(text, pattern);
        assert_eq!(r1, r2, "naive vs kmp");
        assert_eq!(r2, r3, "kmp vs rabin_karp");
        assert_eq!(r3, r4, "rabin_karp vs boyer_moore");
        assert_eq!(r4, r5, "boyer_moore vs z_algorithm");
        assert_eq!(r5, r6, "z_algorithm vs suffix_search");
        r1
    }

    #[test]
    fn test_basic_match() {
        let result = all_agree("abracadabra", "abra");
        assert_eq!(result, vec![0, 7]);
    }

    #[test]
    fn test_single_char_pattern() {
        let result = all_agree("aaa", "a");
        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn test_no_match() {
        let result = all_agree("abcdef", "xyz");
        assert!(result.is_empty());
    }

    #[test]
    fn test_empty_pattern() {
        // Empty pattern matches at every position (including end)
        let result = all_agree("abc", "");
        assert_eq!(result, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_empty_text() {
        let result = all_agree("", "a");
        assert!(result.is_empty());
    }

    #[test]
    fn test_both_empty() {
        let result = all_agree("", "");
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_pattern_longer_than_text() {
        let result = all_agree("ab", "abcdef");
        assert!(result.is_empty());
    }

    #[test]
    fn test_exact_match() {
        let result = all_agree("hello", "hello");
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_overlapping_matches() {
        // "aaa" with "aa" -> matches at 0 and 1 (overlapping)
        let result = all_agree("aaa", "aa");
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_repeated_pattern() {
        let result = all_agree("abcabcabc", "abc");
        assert_eq!(result, vec![0, 3, 6]);
    }

    #[test]
    fn test_unicode_text() {
        // "café" is 5 bytes (é = 2 bytes), "café café" is 11 bytes total
        let result = all_agree("café café", "café");
        assert_eq!(result, vec![0, 6]);
    }

    #[test]
    fn test_single_char_text() {
        let result = all_agree("a", "a");
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_single_char_text_no_match() {
        let result = all_agree("a", "b");
        assert!(result.is_empty());
    }

    #[test]
    fn test_long_repeating_text() {
        let text = "a".repeat(1000);
        let result = all_agree(&text, "aaa");
        assert_eq!(result.len(), 998);
    }

    #[test]
    fn test_pattern_at_end() {
        let result = all_agree("abcdefgh", "fgh");
        assert_eq!(result, vec![5]);
    }

    #[test]
    fn test_pattern_at_start() {
        let result = all_agree("abcdefgh", "abc");
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_all_same_char() {
        let result = all_agree("aaaa", "aa");
        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn test_convenience_wrappers() {
        let text = "test test test";
        let pattern = "test";
        assert_eq!(find_naive(text, pattern), vec![0, 5, 10]);
        assert_eq!(find_kmp(text, pattern), vec![0, 5, 10]);
        assert_eq!(find_rabin_karp(text, pattern), vec![0, 5, 10]);
        assert_eq!(find_boyer_moore(text, pattern), vec![0, 5, 10]);
        assert_eq!(find_z(text, pattern), vec![0, 5, 10]);
        assert_eq!(find_suffix(text, pattern), vec![0, 5, 10]);
    }

    #[test]
    fn test_naive_overlapping_disallow() {
        let result = naive::find_with_overlap("aaa", "aa", overlap::OverlapMode::Disallow);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_kmp_failure_function() {
        let lps = kmp::compute_lps("aabaaab");
        assert_eq!(lps, vec![0, 1, 0, 1, 2, 2, 3]);
    }

    #[test]
    fn test_rabin_karp_hash() {
        let h = rabin_karp::rolling_hash("abc");
        assert_ne!(h, 0);
    }

    #[test]
    fn test_boyer_moore_bad_char_table() {
        let table = boyer_moore::build_bad_char_table("abc");
        assert_eq!(table.len(), 256);
    }

    #[test]
    fn test_z_function() {
        let z = z_algorithm::z_function("aabxaab");
        // z-array for "aabxaab": [7, 1, 0, 0, 3, 1, 0]
        assert_eq!(z[0], 7);
        assert_eq!(z[4], 3);
    }

    #[test]
    fn test_suffix_array_construction() {
        let sa = suffix_search::build_suffix_array("banana");
        assert_eq!(sa, vec![5, 3, 1, 0, 4, 2]);
    }

    #[test]
    fn test_binary_search_in_suffixes() {
        let text = "banana";
        let sa = suffix_search::build_suffix_array(text);
        let found = suffix_search::search_suffix_array(text, &sa, "ana");
        assert_eq!(found, vec![1, 3]);
    }

    #[test]
    fn test_repeat_in_middle() {
        let result = all_agree("xyzABCxyz", "xyz");
        assert_eq!(result, vec![0, 6]);
    }

    #[test]
    fn test_large_alphabet_pattern() {
        let result = all_agree("abcdefghijklmnopqrstuvwxyz", "mnop");
        assert_eq!(result, vec![12]);
    }
}
