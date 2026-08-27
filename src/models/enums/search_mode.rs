use aho_corasick::AhoCorasick;
use regex::Regex;

//This whole enum is to support quicker case insensitive search_term
//it turns the search term into a regex in case the case should #![no_std]
//be considered.
pub enum SearchMode {
    Sensitive((AhoCorasick, usize)),
    Insensitive(Vec<Regex>),
}

impl SearchMode {
    pub fn new(search_terms: Vec<String>, case_insensitive: bool) -> Self {
        if case_insensitive {
            return SearchMode::Insensitive(
                search_terms
                    .iter()
                    .map(|search_term| {
                        let escaped = regex::escape(&search_term);
                        let prepped_term = Regex::new(&format!("(?i){}", escaped))
                            .expect("This should not have happened as the string was escaped.");
                        return prepped_term;
                    })
                    .collect(),
            );
        }
        let term_ammount = search_terms.len();
        return SearchMode::Sensitive((
            AhoCorasick::new(search_terms)
                .expect("Well ... this is awkward. The Aho should've been built"),
            term_ammount,
        ));
    }

    // Now the three functions to determine if there is an and, or or none match are here
    pub fn matches_any(&self, line: &str) -> bool {
        match self {
            SearchMode::Sensitive((aho, _len)) => {
                return aho.is_match(line);
            }
            SearchMode::Insensitive(value) => {
                return value.iter().any(|elem| elem.is_match(line));
            }
        }
    }

    pub fn matches_none(&self, line: &str) -> bool {
        match self {
            SearchMode::Sensitive((aho, _len)) => {
                return !aho.is_match(line);
            }
            SearchMode::Insensitive(value) => {
                return !value.iter().any(|elem| elem.is_match(line));
            }
        }
    }

    pub fn matches_all(&self, line: &str) -> bool {
        match self {
            SearchMode::Sensitive((aho, len)) => {
                let len = *len;
                if len == 0 {
                    return true;
                }

                // Fast path: up to 64 terms fit in a single stack u64, no allocation.
                if len <= 64 {
                    let mut seen: u64 = 0;
                    for m in aho.find_overlapping_iter(line) {
                        seen |= 1u64 << m.pattern().as_usize();
                    }
                    let all = if len == 64 {
                        u64::MAX
                    } else {
                        (1u64 << len) - 1
                    };
                    return seen == all;
                }

                // Fallback: more than 64 terms need a multi-word bitset.
                let words = (len + 63) / 64;
                let mut seen = vec![0u64; words];
                for m in aho.find_overlapping_iter(line) {
                    let i = m.pattern().as_usize();
                    seen[i / 64] |= 1u64 << (i % 64);
                }

                // Every word must be fully set, except the last, which is masked
                // to the exact number of remaining bits.
                let last = words - 1;
                let tail = len % 64;
                let last_mask = if tail == 0 {
                    u64::MAX
                } else {
                    (1u64 << tail) - 1
                };
                for (idx, word) in seen.iter().enumerate() {
                    let expected = if idx == last { last_mask } else { u64::MAX };
                    if *word != expected {
                        return false;
                    }
                }
                return true;
            }
            SearchMode::Insensitive(value) => {
                return value.iter().all(|elem| elem.is_match(line));
            }
        }
    }
}
