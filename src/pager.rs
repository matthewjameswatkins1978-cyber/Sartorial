use std::env;
use std::io::{self, Write};
use std::process::{Command, Stdio};

/// Paging policy for long terminal output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PagerMode {
    /// Page only when attached to an attended interactive TTY and output exceeds terminal height.
    #[default]
    Auto,
    /// Always attempt to pipe output through a pager.
    Always,
    /// Never pipe output through a pager; stream directly.
    Never,
}

impl PagerMode {
    /// Determines whether output should be directed through a pager.
    pub fn should_page(
        &self,
        is_tty: bool,
        is_agent: bool,
        is_plain: bool,
        line_count: usize,
        term_height: usize,
    ) -> bool {
        // Never trap agent JSON or plain machine output in an interactive pager
        if is_agent || is_plain {
            return false;
        }

        match self {
            Self::Never => false,
            Self::Always => true,
            Self::Auto => is_tty && line_count > term_height && term_height > 0,
        }
    }

    /// Emit content to terminal, paging through $PAGER if warranted by policy.
    pub fn page_or_print(
        &self,
        content: &str,
        is_tty: bool,
        is_agent: bool,
        is_plain: bool,
    ) -> io::Result<()> {
        let term_height = crossterm::terminal::size()
            .map(|(_, h)| h as usize)
            .unwrap_or(24);
        let line_count = content.lines().count();

        if !self.should_page(is_tty, is_agent, is_plain, line_count, term_height) {
            let mut out = io::stdout();
            out.write_all(content.as_bytes())?;
            out.flush()?;
            return Ok(());
        }

        // Determine pager binary
        let pager_cmd = env::var("PAGER").unwrap_or_else(|_| {
            if cfg!(windows) {
                "more".to_string()
            } else {
                "less -R".to_string()
            }
        });

        let parts: Vec<&str> = pager_cmd.split_whitespace().collect();
        if parts.is_empty() {
            let mut out = io::stdout();
            out.write_all(content.as_bytes())?;
            out.flush()?;
            return Ok(());
        }

        let mut cmd = Command::new(parts[0]);
        for arg in &parts[1..] {
            cmd.arg(arg);
        }

        cmd.stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        match cmd.spawn() {
            Ok(mut child) => {
                if let Some(mut stdin) = child.stdin.take() {
                    let _ = stdin.write_all(content.as_bytes());
                    let _ = stdin.flush();
                }
                let _ = child.wait();
                Ok(())
            }
            Err(_) => {
                // Graceful fallback to stdout if pager command fails to spawn
                let mut out = io::stdout();
                out.write_all(content.as_bytes())?;
                out.flush()?;
                Ok(())
            }
        }
    }
}
