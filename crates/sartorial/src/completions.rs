#[cfg(feature = "completions")]
use clap::Command;
#[cfg(feature = "completions")]
use clap_complete::{generate, Shell};
#[cfg(feature = "completions")]
use std::io::Write;

/// Supported shell targets for completion script generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportedShell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
}

#[cfg(feature = "completions")]
impl From<SupportedShell> for Shell {
    fn from(s: SupportedShell) -> Self {
        match s {
            SupportedShell::Bash => Shell::Bash,
            SupportedShell::Zsh => Shell::Zsh,
            SupportedShell::Fish => Shell::Fish,
            SupportedShell::PowerShell => Shell::PowerShell,
        }
    }
}

/// Generate a shell completion script for a clap Command to the provided output buffer.
#[cfg(feature = "completions")]
pub fn generate_completion_script(
    cmd: &mut Command,
    shell: SupportedShell,
    bin_name: &str,
    out: &mut dyn Write,
) {
    let clap_shell: Shell = shell.into();
    generate(clap_shell, cmd, bin_name, out);
}
