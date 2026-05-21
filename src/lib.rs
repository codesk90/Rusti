#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppConfig {
    pub name: String,
    pub tagline: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: "Rusti".to_string(),
            tagline: "Rust terminal workbench for AI coding agents".to_string(),
        }
    }
}

pub fn render_welcome(config: &AppConfig) -> String {
    format!(
        "{name}\n{underline}\n{tagline}\n\nCommands:\n  rusti doctor  Check local terminal capabilities\n  rusti demo    Open the starter terminal screen\n",
        name = config.name,
        underline = "=".repeat(config.name.chars().count()),
        tagline = config.tagline,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_names_rusti() {
        let config = AppConfig::default();

        assert_eq!(config.name, "Rusti");
        assert!(config.tagline.contains("terminal workbench"));
    }

    #[test]
    fn welcome_screen_includes_core_commands() {
        let welcome = render_welcome(&AppConfig::default());

        assert!(welcome.contains("Rusti"));
        assert!(welcome.contains("rusti doctor"));
        assert!(welcome.contains("rusti demo"));
    }
}
