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

    /// Format using an already-resolved OSC-8 capability.
    ///
    /// Render paths should prefer this method with `ctx.caps.hyperlinks` so
    /// capability detection happens once rather than being re-sniffed.
    pub fn format_resolved(
        text: &str,
        url: &str,
        is_tty: bool,
        is_plain: bool,
        hyperlinks_supported: bool,
    ) -> String {
        if is_plain || !is_tty || !hyperlinks_supported {
            return format!("{text} ({url})");
        }

        // OSC 8 escape sequence.
        format!("\x1b]8;;{url}\x1b\\{text}\x1b]8;;\x1b\\")
    }

    /// Backwards-compatible convenience helper that performs live detection.
    /// New rendering code should use `format_resolved` with the capability
    /// snapshot held by RenderContext.
    pub fn format(text: &str, url: &str, is_tty: bool, is_plain: bool) -> String {
        Self::format_resolved(text, url, is_tty, is_plain, Self::is_supported())
    }
}
