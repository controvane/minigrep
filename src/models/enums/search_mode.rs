use aho_corasick::{AhoCorasick, AhoCorasickBuilder};

// Search terms are pre-compiled once into a matching strategy. Few-term
// queries use plain `str::contains` (SIMD); many-term queries use a single
// Aho-Corasick pass. Case-insensitive queries lowercase the line for the
// `contains` path, and use an ASCII case-insensitive automaton for the
// many-term path.
//
// Thresholds were found empirically: below them
// `contains` (or `to_lowercase` + `contains`) beats the automaton, above them
// the single pass wins.
const SENSITIVE_THRESHOLD: usize = 8;
const INSENSITIVE_THRESHOLD: usize = 4;

pub enum SearchMode {
    Sensitive {
        terms: Vec<String>,
        aho: AhoCorasick,
    },
    Insensitive {
        terms_lower: Vec<String>,
        aho: AhoCorasick, // built with ascii_case_insensitive
    },
}

impl SearchMode {
    pub fn new(search_terms: Vec<String>, case_insensitive: bool) -> Self {
        if case_insensitive {
            let terms_lower: Vec<String> = search_terms.iter().map(|t| t.to_lowercase()).collect();
            let aho = AhoCorasickBuilder::new()
                .ascii_case_insensitive(true)
                .build(search_terms)
                .expect("Failed to build the Aho-Corasick automaton.");
            return SearchMode::Insensitive { terms_lower, aho };
        }
        let aho = AhoCorasick::new(search_terms.clone())
            .expect("Failed to build the Aho-Corasick automaton.");
        return SearchMode::Sensitive {
            terms: search_terms,
            aho,
        };
    }

    // Now the three functions to determine if there is an and, or or none match are here
    pub fn matches_any(&self, line: &str) -> bool {
        match self {
            SearchMode::Sensitive { terms, aho } => {
                if terms.len() <= SENSITIVE_THRESHOLD {
                    return terms.iter().any(|t| line.contains(t));
                }
                return aho.is_match(line);
            }
            SearchMode::Insensitive { terms_lower, aho } => {
                if terms_lower.len() <= INSENSITIVE_THRESHOLD {
                    let lower = line.to_lowercase();
                    return terms_lower.iter().any(|t| lower.contains(t));
                }
                return aho.is_match(line);
            }
        }
    }

    pub fn matches_none(&self, line: &str) -> bool {
        return !self.matches_any(line);
    }

    pub fn matches_all(&self, line: &str) -> bool {
        match self {
            SearchMode::Sensitive { terms, .. } => {
                return terms.iter().all(|t| line.contains(t));
            }
            SearchMode::Insensitive { terms_lower, .. } => {
                let lower = line.to_lowercase();
                return terms_lower.iter().all(|t| lower.contains(t));
            }
        }
    }
}
