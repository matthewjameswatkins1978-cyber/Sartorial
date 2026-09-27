use crate::components::choice::ChoiceOutcome;
use crate::components::confirm::ConfirmOutcome;

/// Documented semantic exit contract according to the Biscuit Logic standard.
///
/// Numeric choices adhere to cross-platform CLI conventions:
/// - `0`: Success (POSIX standard, affirmative response).
/// - `1`: Declined (User explicitly denied or answered negatively, e.g. confirm "no").
/// - `2`: Usage / CLI parsing / protocol version error (GNU/sysexits standard for incorrect command syntax).
/// - `3`: Requested capability or dependency unavailable (e.g. tool missing from PATH).
/// - `4`: Operation / verification failed, or non-interactive request denied without fallback.
/// - `130`: Interrupted via SIGINT / Cancelled (`128 + 2`, e.g. Esc or Ctrl+C).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    /// Operation completed successfully or confirmed affirmatively.
    Success = 0,
    /// User explicitly declined or answered negatively.
    Declined = 1,
    /// Invalid request, missing flag, protocol violation, or bad command line arguments.
    UsageError = 2,
    /// Requested capability, command, or environment dependency is unavailable.
    Unavailable = 3,
    /// Verification check, core operation failed, or non-interactive request denied without fallback.
    Failed = 4,
    /// Operation was cancelled or interrupted by user (e.g. Ctrl+C, Esc).
    Cancelled = 130,
}

impl ExitCode {
    /// Return the raw integer exit code.
    pub const fn as_i32(self) -> i32 {
        self as i32
    }

    /// Convert into standard Rust process ExitCode.
    pub fn to_process_exit_code(self) -> std::process::ExitCode {
        std::process::ExitCode::from(self as u8)
    }

    /// Exit the process immediately with this code.
    pub fn exit_process(self) -> ! {
        std::process::exit(self as i32)
    }

    /// Map ConfirmOutcome into canonical ExitCode authority.
    pub fn from_confirm_outcome(outcome: &ConfirmOutcome) -> Self {
        match outcome {
            ConfirmOutcome::Confirmed => Self::Success,
            ConfirmOutcome::Denied => Self::Declined,
            ConfirmOutcome::NonInteractiveFallback(true) => Self::Success,
            ConfirmOutcome::NonInteractiveFallback(false) => Self::Declined,
            ConfirmOutcome::NonInteractiveDenied => Self::Failed,
            ConfirmOutcome::Cancelled => Self::Cancelled,
        }
    }

    /// Map ChoiceOutcome into canonical ExitCode authority.
    pub fn from_choice_outcome(outcome: &ChoiceOutcome) -> Self {
        match outcome {
            ChoiceOutcome::Selected(_) => Self::Success,
            ChoiceOutcome::NonInteractiveFallback(_) => Self::Success,
            ChoiceOutcome::NonInteractiveDenied => Self::Failed,
            ChoiceOutcome::Cancelled => Self::Cancelled,
        }
    }
}

impl From<ExitCode> for i32 {
    fn from(code: ExitCode) -> Self {
        code.as_i32()
    }
}
