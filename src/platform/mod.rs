use std::{
    process::{Child, ExitStatus},
    time::Duration,
};

use crate::error::Error;

#[cfg(windows)]
mod windows;

#[cfg(unix)]
mod unix;

/// Sends a Ctrl+C signal to a process.
///
/// # Parameters
///
/// * `child` - The child process to which the Ctrl+C signal will be sent.
///
/// # Returns
///
/// This function returns `Ok` if the signal was sent successfully, otherwise it returns a `Error`.
///
/// # Errors
///
/// This function will return an error in the following situations:
///
/// * `proc_guard::Error::FailedToSendCtrlC` - with a system specific error code if the internal OS API failed.
///
/// # Platform-specific behavior
///
/// - On Windows, it uses `GenerateConsoleCtrlEvent` to send the Ctrl+C signal.
/// - On Unix-like systems, it uses `kill(SIGINT)` to send the Ctrl+C signal.
///
/// # Examples
///
/// ```rust
/// # use std::error::Error;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use std::process::{Command, Stdio};
/// use proc_guard::send_ctrl_c;
/// use std::thread;
/// use std::time::Duration;
///
/// let mut child = if cfg!(target_os = "windows") {
///     Command::new("timeout").args(["/t", "2"]).spawn()?
/// } else {
///     Command::new("sleep").arg("2").spawn()?
/// };
///
/// thread::sleep(Duration::from_secs(1));
///
/// send_ctrl_c(&mut child).expect("Could not send Ctrl+C");
/// #
/// #     Ok(())
/// # }
/// ```
pub fn send_ctrl_c(child: &mut Child) -> Result<(), Error> {
    #[cfg(windows)]
    {
        windows::_send_ctrl_c(child)
    }

    #[cfg(unix)]
    {
        unix::_send_ctrl_c(child)
    }
}

/// Waits for a process to exit with a timeout.
///
/// # Parameters
///
/// * `child` - The child process to wait for.
/// * `timeout` - The duration to wait for the process to exit.
///
/// # Returns
///
/// This function returns `Ok` with the exit status if the process exited within the timeout, otherwise it returns a `Error`.
///
/// # Errors
///
/// This function will return an error in the following situations:
///
/// * `proc_guard::Error::Timeout` - If the specified timeout duration elapses before the process completes.
///   This error indicates that the process did not finish within the allotted time and the function has
///   returned control to the caller.
///
/// * `proc_guard::Error::WaitFailed` - with a system specific error code if the internal OS API failed.
///
/// * Other `proc_guard::Error` variants - For other errors that might occur while waiting for the process.
///
/// # Platform-specific behavior
///
/// - On Windows, it uses `WaitForSingleObject` to wait for the process.
/// - On Unix-like systems, it uses a loop with `try_wait` to wait for the process.
///
/// # Examples
///
/// ```rust
/// # use std::error::Error;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use std::process::{Command, Stdio};
/// use proc_guard::wait_timeout;
/// use std::time::Duration;
///
/// let mut child = if cfg!(target_os = "windows") {
///     Command::new("timeout").args(["/t", "2"]).spawn()?
/// } else {
///     Command::new("sleep").arg("2").spawn()?
/// };
///
/// wait_timeout(&mut child, Duration::from_secs(5)).expect("Wait timeout failure");
/// #
/// #     Ok(())
/// # }
/// ```
pub fn wait_timeout(child: &mut Child, timeout: Duration) -> Result<ExitStatus, Error> {
    #[cfg(windows)]
    {
        windows::_wait_timeout(child, timeout)
    }

    #[cfg(unix)]
    {
        unix::_wait_timeout(child, timeout)
    }
}
