use crossterm::cursor::{Hide, Show};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, stdout, IsTerminal};

/// RAII Guard that manages raw terminal mode and cursor visibility.
/// Ensures terminal state is unconditionally restored on Drop, even during errors or panics.
pub struct TerminalGuard {
    active: bool,
}

impl TerminalGuard {
    /// Enter raw mode and hide cursor safely.
    pub fn enter() -> io::Result<Self> {
        if !stdout().is_terminal() {
            return Ok(Self { active: false });
        }
        enable_raw_mode()?;
        let _ = execute!(stdout(), Hide);
        Ok(Self { active: true })
    }

    /// Check if terminal guard successfully entered raw mode.
    pub fn is_active(&self) -> bool {
        self.active
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        if self.active {
            let _ = execute!(stdout(), Show);
            let _ = disable_raw_mode();
        }
    }
}
