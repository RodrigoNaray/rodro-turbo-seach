//! Search and ordering are independent of GTK and of the operating system.
#[derive(Debug)]
pub struct SearchEntry {
    pub id: String,
    pub name: String,
    normalized: String,
}

impl SearchEntry {
    pub fn new(id: String, name: String) -> Self {
        let normalized = name.to_lowercase();
        Self {
            id,
            name,
            normalized,
        }
    }
}

pub fn search(entries: &[SearchEntry], query: &str, limit: usize) -> Vec<usize> {
    let query = query.trim().to_lowercase();
    if query.is_empty() || limit == 0 {
        return Vec::new();
    }
    let mut matches: Vec<_> = entries
        .iter()
        .enumerate()
        .filter_map(|(index, entry)| {
            let rank = if entry.normalized == query {
                0
            } else if entry.normalized.starts_with(&query) {
                1
            } else if entry.normalized.contains(&query) {
                2
            } else {
                return None;
            };
            Some((rank, &entry.normalized, &entry.id, index))
        })
        .collect();
    matches.sort_unstable();
    matches
        .into_iter()
        .take(limit)
        .map(|(_, _, _, index)| index)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn entries() -> Vec<SearchEntry> {
        [
            ("z", "Visual Studio Code"),
            ("b", "Code Editor"),
            ("a", "Code"),
            ("c", "Firefox"),
            ("d", "Code Editor"),
        ]
        .into_iter()
        .map(|(id, name)| SearchEntry::new(id.into(), name.into()))
        .collect()
    }
    #[test]
    fn ranks_exact_prefix_then_substring_and_breaks_ties_by_id() {
        assert_eq!(search(&entries(), "  CODE  ", 7), [2, 1, 4, 0]);
    }
    #[test]
    fn handles_empty_missing_and_limited_results() {
        assert!(search(&entries(), "", 2).is_empty());
        assert!(search(&entries(), "   ", 2).is_empty());
        assert!(search(&entries(), "absent", 7).is_empty());
        assert!(search(&entries(), "code", 0).is_empty());
        assert_eq!(search(&entries(), "fire", 7), [3]);
    }
}
