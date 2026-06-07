//! Rabin-Karp string matching algorithm.
//!
//! Uses a rolling hash to achieve O(n + m) average-case time complexity.
//! Worst case is O(n·m) due to hash collisions, but this is rare with a good hash function.
//! Uses a large prime modulus to minimize spurious hits.

use crate::Matches;

const BASE: u64 = 257;
const MOD: u64 = 1_000_000_007;

/// Compute the rolling hash of a byte string.
///
/// Uses polynomial rolling hash: `hash(s) = sum(s[i] * BASE^(m-1-i)) % MOD`.
///
/// # Examples
///
/// ```
/// use string_search_rs::rabin_karp;
///
/// let h = rabin_karp::rolling_hash("abc");
/// assert!(h > 0);
/// ```
pub fn rolling_hash(s: &str) -> u64 {
    let bytes = s.as_bytes();
    let mut hash: u64 = 0;
    for &b in bytes {
        hash = (hash * BASE + b as u64) % MOD;
    }
    hash
}

/// Precompute `BASE^m % MOD` for rolling hash updates.
fn compute_power(m: usize) -> u64 {
    let mut pow = 1u64;
    for _ in 0..m {
        pow = (pow * BASE) % MOD;
    }
    pow
}

/// Search for all occurrences of `pattern` in `text` using the Rabin-Karp algorithm.
///
/// Returns a sorted vector of zero-based starting indices.
/// Spurious hits are eliminated by direct string comparison.
///
/// # Examples
///
/// ```
/// use string_search_rs::rabin_karp;
///
/// let matches = rabin_karp::find("the quick brown fox", "fox");
/// assert_eq!(matches, vec![16]);
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

    let pat_hash = rolling_hash(pattern);
    let text_bytes = text.as_bytes();
    let pat_bytes = pattern.as_bytes();
    let power = compute_power(m - 1);

    // Compute initial window hash for text[0..m]
    let mut window_hash: u64 = 0;
    for &b in text_bytes.iter().take(m) {
        window_hash = (window_hash * BASE + b as u64) % MOD;
    }

    let mut result = Vec::new();

    // Check first window
    if window_hash == pat_hash && text_bytes[..m] == pat_bytes[..] {
        result.push(0);
    }

    // Slide the window one byte at a time
    for i in 1..=(n - m) {
        // Remove leading character: hash -= leading * BASE^(m-1)
        let leading = (text_bytes[i - 1] as u64 * power) % MOD;
        window_hash = (window_hash + MOD - leading) % MOD;
        // Shift and add trailing character
        window_hash = (window_hash * BASE + text_bytes[i + m - 1] as u64) % MOD;

        if window_hash == pat_hash && text_bytes[i..i + m] == pat_bytes[..] {
            result.push(i);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_deterministic() {
        assert_eq!(rolling_hash("hello"), rolling_hash("hello"));
    }

    #[test]
    fn test_hash_different() {
        assert_ne!(rolling_hash("hello"), rolling_hash("world"));
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
