use std::process::Child;

use crate::error::Error;

#[cfg(windows)]
#[path = "windows.rs"]
mod imp;

#[cfg(unix)]
#[path = "unix.rs"]
mod imp;

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
    imp::_send_ctrl_c(child)
}
