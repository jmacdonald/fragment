use std::clone::Clone;
use std::cmp::Ordering;
use std::ops::Deref;

#[derive(Debug, PartialEq)]
pub struct Result<T> {
    object: T,
    pub score: f32,
}

impl<T> Deref for Result<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.object
    }
}

pub trait AsStr {
    fn as_str(&self) -> &str;
}

impl AsStr for String {
    fn as_str(&self) -> &str {
        self.as_str()
    }
}

impl<'a> AsStr for &'a str {
    fn as_str(&self) -> &str {
        self
    }
}

/// Given a set, `find` compares its elements and returns a set of `Result`
/// objects ordered by increasing score values (first values are closest
/// matches). If the result set is larger than `max_results`, the set is
/// reduced to that size.
///
/// # Examples
///
/// ```rust
/// use fragment::matching::find;
///
/// let entries = vec![
///     "fragment.rs",
///     "lib.rs"
/// ];
/// let matches = find("lib", &entries, 1);
///
/// assert_eq!(*matches[0], "lib.rs");
/// ```
pub fn find<T: AsStr + Clone>(needle: &str, haystack: &Vec<T>, max_results: usize) -> Vec<Result<T>> {
    let mut results = Vec::new();

    // Calculate a score for each of the haystack entries.
    for object in haystack.into_iter() {
        let score = similarity(needle, object.as_str());

        if score > 0.0 {
          results.push(Result{
              object: object.clone(),
              score: score
          });
        }
    }

    // Sort the results in ascending order (higher values are worse).
    results.sort_by(|a, b| {
        if a.score > b.score {
            Ordering::Less
        } else if a.score < b.score {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    // Make sure we don't exceed the specified result limit.
    results.truncate(max_results);

    results
}

/// Looks for space delimited terms in `query` that occur in `data`,
/// returning a score between 0 and 1, based on the percentage of letters covered
/// in data. Queries with terms that do not exist in `data` return a score of 0.
pub fn similarity(query: &str, data: &str) -> f32 {
    let mut score: f32 = 0.0;

    // Step through all of the query's terms.
    for term in query.split(" ") {
        let mut found = false;

        // Look for term matches in data.
        for (byte_index, _) in data.char_indices() {
            if data[byte_index..].starts_with(term) {
                // Match found; increase score relative to term size.
                score += term.len() as f32/data.len() as f32;

                // Track that we've found a match for this term.
                found = true;
            }
        }

        // Return zero whenever a query term cannot be found in data.
        if !found { return 0.0 }
    }

    // Overlapping matches can produce a score larger than 1.0. Normalize these values.
    score.min(1.0)
}

#[cfg(test)]
mod tests {
    use super::find;

    #[test]
    fn find_returns_a_correctly_ordered_set_of_results() {
        let haystack = vec![
            "src/fragment.rs",
            "lib/fragments.rs"
        ];
        let expected_results = vec![
            "src/fragment.rs",
            "lib/fragments.rs"
        ];
        let results = find("frag", &haystack, 2);
        for i in 0..2 {
            assert_eq!(results[i].object, expected_results[i]);
        }
    }

    #[test]
    fn find_returns_a_correctly_limited_set_of_results() {
        let haystack = vec![
            "src/fragment.rs",
            "lib/fragments.rs"
        ];
        let results = find("fragment", &haystack, 1);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn find_drops_zero_value_results() {
        let haystack = vec![
            "src/fragment.rs",
            "lib/fragments.rs",
            "Fragfile"
        ];
        let results = find("z", &haystack, 3);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn similarity_scores_correctly_when_there_are_no_similarities() {
        assert_eq!(super::similarity("frag", "ment"), 0.0);
    }

    #[test]
    fn similarity_scores_correctly_when_there_is_an_exact_match() {
        assert_eq!(super::similarity("fragment", "fragment"), 1.0);
    }

    #[test]
    fn similarity_scores_correctly_when_there_is_a_half_match() {
        assert_eq!(super::similarity("frag", "fragment"), 0.5);
    }

    #[test]
    fn similarity_sums_term_matches() {
        assert_eq!(super::similarity("frag ment", "fragment"), 1.0);
    }

    #[test]
    fn similarity_limits_score_to_1() {
        assert_eq!(super::similarity("lol", "lololol"), 1.0);
    }

    #[test]
    fn similarity_returns_zero_when_there_are_unmatched_terms() {
        assert_eq!(super::similarity("fg ment", "fragment"), 0.0);
    }
}
