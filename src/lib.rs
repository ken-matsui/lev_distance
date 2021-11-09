//! This code is released under the MIT license
//! # Rust Compiler
//!
//! Permission is hereby granted, free of charge, to any
//! person obtaining a copy of this software and associated
//! documentation files (the "Software"), to deal in the
//! Software without restriction, including without
//! limitation the rights to use, copy, modify, merge,
//! publish, distribute, sublicense, and/or sell copies of
//! the Software, and to permit persons to whom the Software
//! is furnished to do so, subject to the following
//! conditions:
//!
//! The above copyright notice and this permission notice
//! shall be included in all copies or substantial portions
//! of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
//! ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
//! TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
//! PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
//! SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
//! CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
//! OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
//! IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
//! DEALINGS IN THE SOFTWARE.

//! Levenshtein distances.
//!
//! The [Levenshtein distance] is a metric for measuring the difference between two strings.
//!
//! [Levenshtein distance]: https://en.wikipedia.org/wiki/Levenshtein_distance

use std::cmp;

/// Finds the Levenshtein distance between two strings.
pub fn lev_distance(a: &str, b: &str) -> usize {
    // cases which don't require further computation
    if a.is_empty() {
        return b.chars().count();
    } else if b.is_empty() {
        return a.chars().count();
    }

    let mut dcol: Vec<_> = (0..=b.len()).collect();
    let mut t_last = 0;

    for (i, sc) in a.chars().enumerate() {
        let mut current = i;
        dcol[0] = current + 1;

        for (j, tc) in b.chars().enumerate() {
            let next = dcol[j + 1];
            if sc == tc {
                dcol[j + 1] = current;
            } else {
                dcol[j + 1] = cmp::min(current, next);
                dcol[j + 1] = cmp::min(dcol[j + 1], dcol[j]) + 1;
            }
            current = next;
            t_last = j;
        }
    }
    dcol[t_last + 1]
}

/// Finds the best match for a given word in the given iterator.
///
/// As a loose rule to avoid the obviously incorrect suggestions, it takes
/// an optional limit for the maximum allowable edit distance, which defaults
/// to one-third of the given word.
///
/// Besides Levenshtein, we use case insensitive comparison to improve accuracy
/// on an edge case with a lower(upper)case letters mismatch.
pub fn find_best_match_for_name<T>(
    iter_names: impl Iterator<Item = T> + Clone,
    lookup: &str,
    dist: Option<usize>,
) -> Option<String>
where
    T: AsRef<str>,
{
    let max_dist = dist.unwrap_or_else(|| cmp::max(lookup.len(), 3) / 3);

    // Priority of matches:
    // 1. Exact case insensitive match
    // 2. Levenshtein distance match
    // 3. Sorted word match

    // 1. Exact case insensitive match
    for candidate in iter_names.clone() {
        if candidate.as_ref().to_uppercase() == lookup.to_uppercase() {
            return Some(candidate.as_ref().to_string());
        }
    }

    // 2. Levenshtein distance match
    let levenshtein_match = iter_names
        .clone()
        .filter_map(|name| {
            let dist = lev_distance(lookup, name.as_ref());
            if dist <= max_dist {
                Some((name, dist))
            } else {
                None
            }
        })
        // Here we are collecting the next structure:
        // (levenshtein_match, levenshtein_distance)
        .fold(None, |result, (candidate, dist)| match result {
            None => Some((candidate, dist)),
            Some((c, d)) => Some(if dist < d { (candidate, dist) } else { (c, d) }),
        });

    // 3. Sorted word match
    if levenshtein_match.is_some() {
        levenshtein_match.map(|(candidate, _)| candidate.as_ref().to_string())
    } else {
        find_match_by_sorted_words(iter_names, lookup)
    }
}

fn find_match_by_sorted_words<T>(
    iter_names: impl Iterator<Item = T>,
    lookup: &str,
) -> Option<String>
where
    T: AsRef<str>,
{
    iter_names.fold(None, |result, candidate| {
        if sort_by_words(candidate.as_ref()) == sort_by_words(lookup) {
            Some(candidate.as_ref().to_string())
        } else {
            result
        }
    })
}

fn sort_by_words(name: &str) -> String {
    let mut split_words: Vec<&str> = name.split('_').collect();
    // We are sorting primitive &strs and can use unstable sort here.
    split_words.sort_unstable();
    split_words.join("_")
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_lev_distance() {
        use std::char::{from_u32, MAX};
        // Test bytelength agnosticity
        for c in (0..MAX as u32).filter_map(from_u32).map(|i| i.to_string()) {
            assert_eq!(lev_distance(&c[..], &c[..]), 0);
        }

        let a = "\nMäry häd ä little lämb\n\nLittle lämb\n";
        let b = "\nMary häd ä little lämb\n\nLittle lämb\n";
        let c = "Mary häd ä little lämb\n\nLittle lämb\n";
        assert_eq!(lev_distance(a, b), 1);
        assert_eq!(lev_distance(b, a), 1);
        assert_eq!(lev_distance(a, c), 2);
        assert_eq!(lev_distance(c, a), 2);
        assert_eq!(lev_distance(b, c), 1);
        assert_eq!(lev_distance(c, b), 1);
    }

    #[test]
    fn test_find_best_match_for_name() {
        let input = vec!["aaab", "aaabc"];
        assert_eq!(
            find_best_match_for_name(input.iter(), "aaaa", None),
            Some("aaab".to_string())
        );

        assert_eq!(
            find_best_match_for_name(input.iter(), "1111111111", None),
            None
        );

        let input = vec!["AAAA"];
        assert_eq!(
            find_best_match_for_name(input.iter(), "aaaa", None),
            Some("AAAA".to_string())
        );

        let input = vec!["AAAA"];
        assert_eq!(
            find_best_match_for_name(input.iter(), "aaaa", Some(4)),
            Some("AAAA".to_string())
        );

        let input = vec!["a_longer_variable_name"];
        assert_eq!(
            find_best_match_for_name(input.iter(), "a_variable_longer_name", None),
            Some("a_longer_variable_name".to_string())
        );
    }
}
