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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalConfig {
    pub shell: String,
    pub title: String,
}

impl TerminalConfig {
    pub fn from_shell_env(shell: Option<&str>) -> Self {
        let shell = match shell {
            Some(value) if !value.trim().is_empty() => value.to_string(),
            _ => default_shell().to_string(),
        };

        Self {
            shell,
            title: "Rusti Terminal".to_string(),
        }
    }

    pub fn shell_name(&self) -> &str {
        self.shell.rsplit('/').next().unwrap_or(&self.shell)
    }
}

#[cfg(target_os = "macos")]
fn default_shell() -> &'static str {
    "/bin/zsh"
}

#[cfg(not(target_os = "macos"))]
fn default_shell() -> &'static str {
    "/bin/sh"
}

pub fn render_welcome(config: &AppConfig) -> String {
    format!(
        "{name}\n{underline}\n{tagline}\n\nCommands:\n  rusti terminal  Open a real terminal session backed by your shell\n  rusti doctor    Check local terminal capabilities\n  rusti demo      Open the starter terminal screen\n",
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
        assert!(welcome.contains("rusti terminal"));
        assert!(welcome.contains("rusti doctor"));
        assert!(welcome.contains("rusti demo"));
    }

    #[test]
    fn terminal_config_uses_shell_env() {
        let config = TerminalConfig::from_shell_env(Some("/bin/zsh"));

        assert_eq!(config.shell, "/bin/zsh");
        assert_eq!(config.shell_name(), "zsh");
    }

    #[test]
    fn terminal_config_falls_back_to_macos_builtin_shell() {
        let config = TerminalConfig::from_shell_env(Some(""));

        if cfg!(target_os = "macos") {
            assert_eq!(config.shell, "/bin/zsh");
        } else {
            assert_eq!(config.shell, "/bin/sh");
        }
    }
}
