use crate::{State, ThemeEntry};
use indexmap::{IndexMap, IndexSet};
use serde_json::json;

/// Return default State with bundled light/dark themes for relay-runtime and standalone use.
/// Matches theme.yaml from tests: default inherits light/dark, light and dark themes with
/// .bg-primary, .bg-surface, .text-muted, etc.
pub fn bundled_state() -> State {
    let mut themes = IndexMap::new();

    // default: meta-theme that inherits light/dark based on prefers-color-scheme
    let default_entry = ThemeEntry {
        name: Some("default".into()),
        inherits: Some("light".into()),
        inherits_dark: Some("dark".into()),
        selectors: IndexMap::new(),
        variables: IndexMap::new(),
        breakpoints: IndexMap::new(),
    };
    themes.insert("default".into(), default_entry);

    // light theme
    let mut light_vars = IndexMap::new();
    light_vars.insert("primary".into(), "#3b82f6".into());
    light_vars.insert("secondary".into(), "#10b981".into());
    light_vars.insert("bg".into(), "#ffffff".into());
    light_vars.insert("surface".into(), "#f9fafb".into());
    light_vars.insert("text".into(), "#111827".into());
    light_vars.insert("muted".into(), "#6b7280".into());
    light_vars.insert("border".into(), "#e5e7eb".into());

    let mut light_selectors = IndexMap::new();
    light_selectors.insert(
        "body".into(),
        [("backgroundColor".into(), json!("var(bg)"))].into_iter().collect(),
    );
    light_selectors.insert(
        "span, button, h1, h2, h3, h4, h5, h6, p".into(),
        [("color".into(), json!("var(text)"))].into_iter().collect(),
    );
    light_selectors.insert(
        ".bg-primary".into(),
        [("backgroundColor".into(), json!("var(primary)"))].into_iter().collect(),
    );
    light_selectors.insert(
        ".bg-secondary".into(),
        [("backgroundColor".into(), json!("var(secondary)"))].into_iter().collect(),
    );
    light_selectors.insert(
        ".bg-surface".into(),
        [("backgroundColor".into(), json!("var(surface)"))].into_iter().collect(),
    );
    light_selectors.insert(
        ".text-primary".into(),
        [("color".into(), json!("var(primary)"))].into_iter().collect(),
    );
    light_selectors.insert(
        ".text-themed".into(),
        [("color".into(), json!("var(muted)"))].into_iter().collect(),
    );
    light_selectors.insert(
        ".text-muted".into(),
        [("color".into(), json!("var(muted)"))].into_iter().collect(),
    );
    light_selectors.insert(
        ".border-themed".into(),
        [
            ("borderColor".into(), json!("var(border)")),
            ("borderWidth".into(), json!(1)),
        ]
        .into_iter()
        .collect(),
    );

    let light_entry = ThemeEntry {
        name: Some("light".into()),
        inherits: None,
        inherits_dark: None,
        selectors: light_selectors,
        variables: light_vars,
        breakpoints: IndexMap::new(),
    };
    themes.insert("light".into(), light_entry);

    // dark theme (inherits light, overrides variables)
    let mut dark_vars = IndexMap::new();
    dark_vars.insert("bg".into(), "#111827".into());
    dark_vars.insert("surface".into(), "#1f2937".into());
    dark_vars.insert("text".into(), "#f9fafb".into());
    dark_vars.insert("muted".into(), "#9ca3af".into());
    dark_vars.insert("primary".into(), "#60a5fa".into());
    dark_vars.insert("border".into(), "#374151".into());

    let dark_entry = ThemeEntry {
        name: Some("dark".into()),
        inherits: Some("light".into()),
        inherits_dark: None,
        selectors: IndexMap::new(),
        variables: dark_vars,
        breakpoints: IndexMap::new(),
    };
    themes.insert("dark".into(), dark_entry);

    State {
        themes,
        default_theme: "default".into(),
        current_theme: "default".into(),
        prefers_color_scheme: None,
        display_density: 1.0,
        scaled_density: 1.0,
        used_classes: IndexSet::new(),
        used_tags: IndexSet::new(),
        used_tag_classes: IndexSet::new(),
    }
}
