/// Documented semantic exit contract according to the Biscuit Logic standard.
///
/// Numeric choices adhere to cross-platform CLI conventions:
/// - `0`: Success (POSIX standard).
/// - `2`: Usage / CLI parsing error (GNU/sysexits standard for incorrect command syntax).
/// - `3`: Requested capability or dependency unavailable (e.g. tool missing from PATH).
/// - `4`: Operation / verification failed (e.g. test failure, lint failure).
/// - `130`: Interrupted via SIGINT / Cancelled (`128 + 2`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    /// Operation completed successfully.
    Success = 0,
    /// Invalid request, missing flag, or bad command line arguments.
    UsageError = 2,
    /// Requested capability, command, or environment dependency is unavailable.
    Unavailable = 3,
    /// Verification check or core operation failed.
    Failed = 4,
    /// Operation was cancelled or interrupted by user (e.g. Ctrl+C).
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
}

impl From<ExitCode> for i32 {
    fn from(code: ExitCode) -> Self {
        code.as_i32()
    }
}
