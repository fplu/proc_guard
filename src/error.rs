use std::io;
use thiserror;

// note if you have a linter error on "thiserror::Error": https://stackoverflow.com/questions/72698907/proc-macro-not-found

/// `Error` represents the various errors that can occur while handling process guards.
/// This enum derives the `thiserror::Error` and `Debug` traits for error handling and debugging respectively.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Indicates that a timeout has occurred.
    #[error("Timeout occurred")]
    Timeout,

    /// Indicates that sending a Ctrl-C signal failed.
    /// The associated value is the error code from the failed attempt.
    #[error("Failed to send Ctrl-C: {0}")]
    FailedToSendCtrlC(u32),

    /// Indicates that waiting for a process failed.
    /// The associated value is the error code from the failed wait.
    #[error("Wait failed: {0}")]
    WaitFailed(u32),

    /// Indicates an unknown error.
    #[error("Unknown error")]
    UnknownError(),

    /// Indicates that the target process was forcefully terminated.
    #[error("Forcefully terminate")]
    ForcefullyTerminate(),

    /// Indicates an I/O error.
    /// The associated value is the underlying `io::Error`.
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
}
