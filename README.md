# string-search-rs

[![crates.io](https://img.shields.io/crates/v/string-search-rs.svg)](https://crates.io/crates/string-search-rs)

Comprehensive string matching algorithms implemented in pure Rust with no external dependencies.

## Algorithms

| Algorithm | Module | Time Complexity |
|-----------|--------|----------------|
| Naive (brute-force) | `naive` | O(n·m) |
| Knuth-Morris-Pratt | `kmp` | O(n + m) |
| Rabin-Karp | `rabin_karp` | O(n + m) avg |
| Boyer-Moore (bad char) | `boyer_moore` | O(n/m) best |
| Z-algorithm | `z_algorithm` | O(n + m) |
| Suffix Array Search | `suffix_search` | O(n²logn) build, O(mlogn) query |

## Usage

```rust
use string_search_rs::{naive, kmp, rabin_karp, boyer_moore, z_algorithm, suffix_search};

let text = "abracadabra";
let pattern = "abra";

assert_eq!(naive::find(text, pattern), vec![0, 7]);
assert_eq!(kmp::find(text, pattern), vec![0, 7]);
assert_eq!(rabin_karp::find(text, pattern), vec![0, 7]);
assert_eq!(boyer_moore::find(text, pattern), vec![0, 7]);
assert_eq!(z_algorithm::find(text, pattern), vec![0, 7]);
assert_eq!(suffix_search::find(text, pattern), vec![0, 7]);
```

## Features

- Pure Rust, no external dependencies
- Comprehensive documentation and examples
- 25+ tests covering edge cases and algorithmic correctness
- Overlap control for naive matching

## License

MIT OR Apache-2.0
