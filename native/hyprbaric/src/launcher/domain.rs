//! Launcher domain state, ranking, and UI-facing result projection.

use std::{collections::HashMap, fmt, path::PathBuf};

use serde::{Deserialize, Serialize};

use super::{
    icons::IconIndex,
    usage::{UsageInfo, unix_timestamp, usage_bonus},
};

const RESULTS_LIMIT: usize = 12;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Phase {
    Loading,
    Ready,
    Failed { message: String },
}

/// Stable desktop-entry identity.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id(String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResultEntry {
    pub id: Id,
    pub name: String,
    pub subtitle: Option<String>,
    pub icon_name: Option<String>,
    pub icon_path: Option<PathBuf>,
    pub terminal: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Results {
    pub phase: Phase,
    pub query: String,
    pub entries: Vec<ResultEntry>,
}

pub(super) struct State {
    pub(super) phase: Phase,
    pub(super) query: String,
    pub(super) cache: Cache,
}

#[derive(Default)]
pub(super) struct Cache {
    pub(super) entries: Vec<Entry>,
    pub(super) usage: HashMap<Id, UsageInfo>,
    pub(super) icons: Option<IconIndex>,
}

#[derive(Clone, Debug)]
pub(super) struct Entry {
    pub(super) id: Id,
    pub(super) name: String,
    pub(super) subtitle: Option<String>,
    pub(super) icon_name: Option<String>,
    pub(super) icon_path: Option<PathBuf>,
    pub(super) icon_resolved: bool,
    pub(super) terminal: bool,
    pub(super) desktop_path: PathBuf,
    pub(super) exec: String,
    pub(super) working_dir: Option<PathBuf>,
    pub(super) normalized: SearchFields,
}

#[derive(Clone, Debug)]
pub(super) struct SearchFields {
    pub(super) name: String,
    pub(super) exec: String,
    pub(super) subtitle: String,
    pub(super) keywords: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SearchMatch {
    ExactName,
    PrefixName,
    ContainsName,
    PrefixExec,
    ContainsExec,
    ContainsSubtitle,
    ContainsKeyword,
}

impl State {
    pub(super) fn results(&self) -> Results {
        let entries = match self.phase {
            Phase::Loading => Vec::new(),
            Phase::Ready | Phase::Failed { .. } => self.cache.search(&self.query, RESULTS_LIMIT),
        };

        Results {
            phase: self.phase.clone(),
            query: self.query.clone(),
            entries,
        }
    }
}

impl Phase {
    pub fn message(&self) -> Option<&str> {
        match self {
            Self::Failed { message } => Some(message),
            Self::Loading | Self::Ready => None,
        }
    }
}

impl Id {
    pub fn new(value: impl Into<String>) -> Option<Self> {
        let value = value.into();
        (!value.trim().is_empty()).then_some(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Id {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl Cache {
    pub(super) fn entry(&self, id: &Id) -> Option<&Entry> {
        self.entries.iter().find(|entry| &entry.id == id)
    }

    pub(super) fn resolve_visible_icons(&mut self, query: &str) -> bool {
        let indices = self.ranked_indices(query, RESULTS_LIMIT);
        if indices.is_empty() {
            return false;
        }

        let icons = self.icons.get_or_insert_with(IconIndex::new);
        let mut changed = false;

        for index in indices {
            let entry = &mut self.entries[index];
            if entry.icon_resolved {
                continue;
            }

            entry.icon_path = entry
                .icon_name
                .as_deref()
                .and_then(|icon| icons.resolve(icon));
            entry.icon_resolved = true;
            changed = true;
        }

        changed
    }

    pub(super) fn search(&self, query: &str, limit: usize) -> Vec<ResultEntry> {
        self.ranked_indices(query, limit)
            .into_iter()
            .map(|index| self.entries[index].result())
            .collect()
    }

    fn ranked_indices(&self, query: &str, limit: usize) -> Vec<usize> {
        let query = normalize(query);
        let tokens = query
            .split_whitespace()
            .filter(|token| !token.is_empty())
            .collect::<Vec<_>>();
        let now = unix_timestamp();

        let mut ranked = self
            .entries
            .iter()
            .enumerate()
            .filter_map(|entry| {
                let (index, entry) = entry;
                let base_score = if tokens.is_empty() {
                    0
                } else {
                    score_entry(entry, &tokens)?
                };
                let usage_bonus = self
                    .usage
                    .get(&entry.id)
                    .map(|usage| usage_bonus(usage, now))
                    .unwrap_or_default();
                Some((index, base_score + usage_bonus))
            })
            .collect::<Vec<_>>();

        ranked.sort_by(|(left_index, left_score), (right_index, right_score)| {
            let left_entry = &self.entries[*left_index];
            let right_entry = &self.entries[*right_index];
            right_score
                .cmp(left_score)
                .then_with(|| left_entry.name.cmp(&right_entry.name))
                .then_with(|| left_entry.id.cmp(&right_entry.id))
        });

        ranked
            .into_iter()
            .take(limit)
            .map(|(index, _)| index)
            .collect()
    }

    pub(super) fn record_launch(&mut self, entry_id: &Id) {
        let usage = self.usage.entry(entry_id.clone()).or_default();
        usage.launches = usage.launches.saturating_add(1);
        usage.last_used_unix = unix_timestamp();
    }
}

impl Entry {
    fn result(&self) -> ResultEntry {
        ResultEntry {
            id: self.id.clone(),
            name: self.name.clone(),
            subtitle: self.subtitle.clone(),
            icon_name: self.icon_name.clone(),
            icon_path: self.icon_path.clone(),
            terminal: self.terminal,
        }
    }
}

fn score_entry(entry: &Entry, tokens: &[&str]) -> Option<i64> {
    let mut score = 0_i64;
    for token in tokens {
        score += score_token(entry, token)? as i64;
    }
    Some(score)
}

fn score_token(entry: &Entry, token: &str) -> Option<u32> {
    let mut matches = Vec::new();

    if entry.normalized.name == token {
        matches.push(SearchMatch::ExactName);
    } else if entry.normalized.name.starts_with(token) {
        matches.push(SearchMatch::PrefixName);
    } else if entry.normalized.name.contains(token) {
        matches.push(SearchMatch::ContainsName);
    }

    if entry.normalized.exec.starts_with(token) {
        matches.push(SearchMatch::PrefixExec);
    } else if entry.normalized.exec.contains(token) {
        matches.push(SearchMatch::ContainsExec);
    }

    if entry.normalized.subtitle.contains(token) {
        matches.push(SearchMatch::ContainsSubtitle);
    }

    if entry
        .normalized
        .keywords
        .iter()
        .any(|keyword| keyword.contains(token))
    {
        matches.push(SearchMatch::ContainsKeyword);
    }

    matches.into_iter().map(match_score).max()
}

fn match_score(match_type: SearchMatch) -> u32 {
    match match_type {
        SearchMatch::ExactName => 100,
        SearchMatch::PrefixName => 80,
        SearchMatch::ContainsName => 60,
        SearchMatch::PrefixExec => 50,
        SearchMatch::ContainsExec => 45,
        SearchMatch::ContainsSubtitle => 30,
        SearchMatch::ContainsKeyword => 20,
    }
}

pub(super) fn normalize(text: &str) -> String {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

pub(super) fn normalize_field(text: &str) -> Option<String> {
    let value = text.trim();
    (!value.is_empty()).then(|| value.to_string())
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::*;

    fn entry(id: &str, name: &str, exec: &str) -> Entry {
        Entry {
            id: Id::new(id).expect("test entry ID should be non-empty"),
            name: name.to_string(),
            subtitle: Some("Browser".to_string()),
            icon_name: Some("firefox".to_string()),
            icon_path: Some(PathBuf::from(
                "/usr/share/icons/hicolor/scalable/apps/firefox.svg",
            )),
            icon_resolved: true,
            terminal: false,
            desktop_path: Path::new("/usr/share/applications").join(id),
            exec: exec.to_string(),
            working_dir: None,
            normalized: SearchFields {
                name: normalize(name),
                exec: normalize(exec),
                subtitle: normalize("Browser"),
                keywords: vec![normalize("web")],
            },
        }
    }

    #[test]
    fn search_prefers_exact_name_matches() {
        let mut cache = Cache::default();
        cache.entries = vec![
            entry("firefox.desktop", "Firefox", "firefox %u"),
            entry("foot.desktop", "Foot", "foot"),
        ];

        let results = cache.search("firefox", 5);
        assert_eq!(
            results.first().map(|entry| entry.id.as_str()),
            Some("firefox.desktop")
        );
    }

    #[test]
    fn usage_boost_favors_recent_entries_for_empty_query() {
        let mut cache = Cache::default();
        cache.entries = vec![
            entry("older.desktop", "Older", "older"),
            entry("recent.desktop", "Recent", "recent"),
        ];
        cache.usage.insert(
            Id::new("recent.desktop").expect("test entry ID should be non-empty"),
            UsageInfo {
                launches: 4,
                last_used_unix: unix_timestamp(),
            },
        );

        let results = cache.search("", 5);
        assert_eq!(
            results.first().map(|entry| entry.id.as_str()),
            Some("recent.desktop")
        );
    }

    #[test]
    fn results_keep_entries_when_phase_is_failed() {
        let mut cache = Cache::default();
        cache.entries = vec![entry("firefox.desktop", "Firefox", "firefox")];
        let state = State {
            phase: Phase::Failed {
                message: "boom".to_string(),
            },
            query: "fire".to_string(),
            cache,
        };

        let results = state.results();
        assert!(matches!(&results.phase, Phase::Failed { .. }));
        assert_eq!(results.entries.len(), 1);
        assert_eq!(results.phase.message(), Some("boom"));
    }
}
