//! Live terminal interaction. Requires the `interactive` feature
//! (crossterm-backed keyboard handling and raw-mode guards).

#![cfg(feature = "interactive")]

pub mod keyboard;
pub mod terminal;

pub use keyboard::read_key;
pub use terminal::TerminalGuard;
