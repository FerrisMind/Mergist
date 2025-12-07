use globset::{Glob, GlobSet, GlobSetBuilder};

use crate::models::{ConvertOptions, FileEntry};

const LARGE_FILE_THRESHOLD: u64 = 1_048_576; // 1MB

#[derive(Debug)]
pub struct SkipMatcher {
    globset: Option<GlobSet>,
}

impl SkipMatcher {
    pub fn new(patterns: &[String]) -> Self {
        let mut builder = GlobSetBuilder::new();
        for pattern in patterns {
            if pattern.trim().is_empty() {
                continue;
            }
            let mut pat = pattern.trim().to_string();
            // If user specified a directory with trailing slash, match all inside it
            if pat.ends_with('/') {
                pat = format!("{}**", pat.trim_end_matches('/'));
            }
            // If pattern does not start with **/ or /, allow matching in any subdirectory
            if !pat.starts_with("**/") && !pat.starts_with('/') && pat.contains('/') {
                pat = format!("**/{}", pat);
            }
            if let Ok(glob) = Glob::new(&pat) {
                builder.add(glob);
            }
        }
        let globset = builder.build().ok();
        Self { globset }
    }

    pub fn is_match(&self, path: &str) -> bool {
        match &self.globset {
            Some(gs) => gs.is_match(path),
            None => false,
        }
    }
}

pub fn should_skip_file(file: &FileEntry, matcher: &SkipMatcher, options: &ConvertOptions) -> bool {
    if options.skip_large_files && file.size > LARGE_FILE_THRESHOLD {
        return true;
    }

    if file
        .path
        .split('/')
        .next_back()
        .is_some_and(|name| matcher.is_match(name))
    {
        return true;
    }

    matcher.is_match(&file.path)
}

pub fn filter_files<'a>(
    files: &'a [FileEntry],
    matcher: &SkipMatcher,
    options: &ConvertOptions,
) -> Vec<&'a FileEntry> {
    files
        .iter()
        .filter(|f| !should_skip_file(f, matcher, options))
        .collect()
}
