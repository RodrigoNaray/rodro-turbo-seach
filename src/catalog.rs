//! Linux desktop entries and launching through GIO, never a shell.
use crate::search::SearchEntry;
use gtk::{gio, prelude::*};

pub struct Catalog {
    pub entries: Vec<SearchEntry>,
    pub apps: Vec<gio::AppInfo>,
}

impl Catalog {
    pub fn discover() -> Self {
        let mut entries = Vec::new();
        let mut apps = Vec::new();
        for app in gio::AppInfo::all() {
            if !app.should_show() {
                continue;
            }
            let Some(id) = app.id() else {
                continue;
            };
            if id == "io.github.rodro.TurboSearch.desktop" {
                continue;
            }
            entries.push(SearchEntry::new(
                id.to_string(),
                app.display_name().to_string(),
            ));
            apps.push(app);
        }
        Self { entries, apps }
    }
}
