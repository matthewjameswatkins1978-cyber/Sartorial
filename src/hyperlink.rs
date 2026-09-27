use std::env;

/// Tasteful terminal hyperlink formatting following the OSC 8 standard.
/// Ensures plain mode exposes understandable text/URL without invisible link requirements.
pub struct Hyperlink;

impl Hyperlink {
    /// Detect if the current terminal environment likely supports OSC 8 hyperlinks.
    pub fn is_supported() -> bool {
        if env::var_os("DOMTERM").is_some()
            || env::var_os("WT_SESSION").is_some() // Windows Terminal
            || env::var_os("ITERM_SESSION_ID").is_some()
            || env::var_os("VSCODE_INJECTION").is_some()
        {
            return true;
        }

        if let Ok(term_program) = env::var("TERM_PROGRAM") {
            matches!(
                term_program.as_str(),
                "iTerm.app" | "WezTerm" | "vscode" | "Hyper" | "Alacritty"
            )
        } else {
            false
        }
    }

    /// Format a clickable hyperlink with fallback for plain text or unsupported terminals.
    pub fn format(text: &str, url: &str, is_tty: bool, is_plain: bool) -> String {
        if is_plain || !is_tty {
            return format!("{text} ({url})");
        }

        if Self::is_supported() {
            // OSC 8 escape sequence: \x1b]8;;URL\x1b\TEXT\x1b]8;;\x1b\
            format!("\x1b]8;;{url}\x1b\\{text}\x1b]8;;\x1b\\")
        } else {
            format!("{text} ({url})")
        }
    }
}
