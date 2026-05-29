/// Convert dp to pixels using display density
pub fn dp_to_px(dp: f32, density: f32) -> i32 {
    (dp * density).round() as i32
}

/// Convert sp to pixels using scaled density  
pub fn sp_to_px(sp: f32, scaled_density: f32) -> f32 {
    sp * scaled_density
}

/// Parse font-size (rem, px) to Android sp. Font sizes are scale-independent.
/// rem uses 16px base. px and bare numbers map 1:1 to sp.
pub fn parse_font_size_to_sp(value: &serde_json::Value, _density: f32) -> Option<serde_json::Value> {
    const REM_BASE: f32 = 16.0;
    match value {
        serde_json::Value::Number(n) => {
            let sp = n.as_f64()? as f32;
            Some(serde_json::json!(sp))
        }
        serde_json::Value::String(s) => {
            let trimmed = s.trim();
            if trimmed.ends_with("rem") {
                let rem = trimmed.trim_end_matches("rem").trim().parse::<f32>().ok()?;
                Some(serde_json::json!((rem * REM_BASE)))
            } else if trimmed.ends_with("px") {
                let px = trimmed.trim_end_matches("px").trim().parse::<f32>().ok()?;
                Some(serde_json::json!(px))
            } else if let Ok(num) = trimmed.parse::<f32>() {
                Some(serde_json::json!(num))
            } else {
                None
            }
        }
        _ => None
    }
}

/// Parse a CSS value and convert to Android pixels if needed
pub fn parse_and_convert_to_px(value: &serde_json::Value, density: f32) -> Option<serde_json::Value> {
    match value {
        serde_json::Value::Number(n) => {
            // Bare number treated as dp
            let dp = n.as_f64()? as f32;
            Some(serde_json::json!(dp_to_px(dp, density)))
        }
        serde_json::Value::String(s) => {
            // Parse string with units
            let trimmed = s.trim();
            if trimmed.ends_with("px") {
                // Treat px as density-independent pixels (dp) for cross-platform parity
                let px = trimmed.trim_end_matches("px").trim().parse::<f32>().ok()?;
                Some(serde_json::json!(dp_to_px(px, density)))
            } else if trimmed.ends_with("dp") {
                let dp = trimmed.trim_end_matches("dp").trim().parse::<f32>().ok()?;
                Some(serde_json::json!(dp_to_px(dp, density)))
            } else if let Ok(num) = trimmed.parse::<f32>() {
                // Bare number as string, treat as dp
                Some(serde_json::json!(dp_to_px(num, density)))
            } else {
                // Keep as-is (e.g., "wrap_content", "match_parent")
                None
            }
        }
        _ => None
    }
}
