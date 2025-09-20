use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub enum MenuOrder {
    #[default]
    AfterHome,
    BeforeSettings,
    End,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Shortcut {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub menu_order: MenuOrder,
    #[serde(default)]
    pub order: usize,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub target_blank: bool,
    #[serde(default)]
    pub badge: Option<bool>,
    #[serde(default)]
    pub badge_color: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UiConfig {
    #[serde(default)]
    pub shortcut: Option<Vec<Shortcut>>,
    #[serde(default)]
    pub message: Option<String>,
}

impl UiConfig {
    pub fn shortcuts_sorted(&self) -> Vec<Shortcut> {
        let mut v = self.shortcut.clone().unwrap_or_default();
        v.sort_by_key(|s| s.order);
        v
    }
}
