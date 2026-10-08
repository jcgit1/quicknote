use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppState {
    pub tabs: Vec<Tab>,
    pub active: usize,
    pub shrunken: bool,
    #[serde(default)]
    pub last_inner_size: Option<[f32; 2]>,
    #[serde(default)]
    pub last_pos: Option<[f32; 2]>,
    pub bg_color: [u8; 3],
    pub text_color: [u8; 3],
    #[serde(default)]
    pub settings_open: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Tab {
    pub id: u64,
    pub name: String,
    pub text: String,
}

impl Tab {
    pub fn new(id: u64, name: String) -> Self {
        Self { id, name, text: String::new() }
    }
}

pub const DEFAULT_BG: [u8; 3] = [40, 44, 52];
pub const DEFAULT_TEXT: [u8; 3] = [224, 228, 234];

impl Default for AppState {
    fn default() -> Self {
        Self {
            tabs: vec![Tab::new(1, "Notes".into())],
            active: 0,
            shrunken: false,
            last_inner_size: None,
            last_pos: None,
            bg_color: DEFAULT_BG,
            text_color: DEFAULT_TEXT,
            settings_open: false,
        }
    }
}

impl AppState {
    pub fn path() -> PathBuf {
        let base = directories::ProjectDirs::from("", "jcgit1", "quicknote")
            .map(|p| p.data_local_dir().to_path_buf())
            .unwrap_or_else(|| {
                let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
                PathBuf::from(home).join(".local/share/quicknote")
            });
        base.join("state.json")
    }

    pub fn load() -> Self {
        let path = Self::path();
        match fs::read_to_string(&path) {
            Ok(raw) => match serde_json::from_str::<AppState>(&raw) {
                Ok(mut state) => {
                    state.sanitize();
                    state
                }
                Err(_) => Self::default(),
            },
            Err(_) => Self::default(),
        }
    }

    fn sanitize(&mut self) {
        if self.tabs.is_empty() {
            self.tabs.push(Tab::new(1, "Notes".into()));
        }
        if self.active >= self.tabs.len() {
            self.active = 0;
        }
        let mut next_id = self.tabs.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        for tab in self.tabs.iter_mut() {
            if tab.id == 0 {
                tab.id = next_id;
                next_id += 1;
            }
        }
        self.active = self.active.min(self.tabs.len().saturating_sub(1));
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self).expect("state serializes");
        let tmp = path.with_extension("json.tmp");
        fs::write(&tmp, json)?;
        fs::rename(&tmp, &path)?;
        Ok(())
    }
}
