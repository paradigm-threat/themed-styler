use super::{State, ThemeEntry};
use indexmap::IndexMap;
use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct ThemesInput {
    #[serde(default)]
    pub themes: IndexMap<String, ThemeEntry>,
    #[serde(default)]
    pub current_theme: Option<String>,
    #[serde(default, rename = "prefersColorScheme", alias = "prefers_color_scheme")]
    pub prefers_color_scheme: Option<String>,
}

pub fn build_state(themes_input: ThemesInput) -> State {
    let mut state = State::new_default();
    if !themes_input.themes.is_empty() {
        state.themes = themes_input.themes;
        if let Some(current) = themes_input.current_theme.clone() {
            if state.themes.contains_key(&current) {
                state.current_theme = current;
            }
        }
        if state.default_theme.is_empty() {
            if let Some((name, _)) = state.themes.iter().next() {
                state.default_theme = name.clone();
            }
        }
        if let Some(prefers) = themes_input.prefers_color_scheme.as_ref() {
            state.prefers_color_scheme = Some(prefers.to_lowercase());
        }
    }
    state
}

/// Build State from theme JSON. Used by relay-runtime to resolve className→style.
/// JSON format: {"themes":{...},"current_theme":"light","prefersColorScheme":"light"}
pub fn build_state_from_theme_json(json: &str) -> State {
    build_state(parse_themes_json(json))
}

pub fn parse_themes_json(json: &str) -> ThemesInput {
    match serde_json::from_str(json) {
        Ok(input) => input,
        Err(e) => {
            eprintln!("[parse_themes_json] Deserialization error: {}", e);
            ThemesInput::default()
        }
    }
}
