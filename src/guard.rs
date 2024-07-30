use std::{
    io,
    mem::ManuallyDrop,
    process::{Child, Command, ExitStatus},
    time::Duration,
};

use child_wait_timeout::ChildWT;

use crate::{error::Error, send_ctrl_c};

/// Enum representing the various termination strategies available for a process guard.
#[derive(Debug, Clone, Copy)]
pub enum ProcessTermination {
    /// Wait indefinitely for the process to exit.
    Wait,
    /// Wait for a specified duration for the process to exit.
    WaitTimeout(Duration),
    /// Wait for a specified duration, then kill the process if it hasn't exited.
    WaitTimeoutKill(Duration),
    /// Send a Ctrl+C signal to the process and does NOT wait.
    CtrlC,
    /// Send a Ctrl+C signal and wait indefinitely for the process to exit.
    CtrlCWait,
    /// Send a Ctrl+C signal and wait for a specified duration for the process to exit.
    CtrlCWaitTimeout(Duration),
    /// Send a Ctrl+C signal, wait for a specified duration, and then kill the process if it hasn't exited.
    CtrlCWaitTimeoutKill(Duration),
    /// Kill the process immediately and does NOT wait.
    Kill,
    /// Kill the process immediately and wait indefinitely for the process to exit.
    KillWait,
}

/// Struct representing a guard for a child process.
/// Ensures the process is terminated as specified when the guard is dropped.
#[derive(Debug)]
pub struct ProcGuard {
    child: ManuallyDrop<Child>,
    dropped: bool,
    termination: ProcessTermination,
}

impl ProcGuard {
    /// Creates a new `ProcGuard`.
    ///
    /// # Arguments
    ///
    /// * `child` - The child process to guard.
    /// * `termination` - The termination strategy to use when dropping the guard.
    ///
    /// # Returns
    ///
    /// * `ProcGuard` - A guard around the child process.
    ///
    /// # Example
    /// ```
    /// # use std::error::Error;
    /// # use std::thread;
    /// # use std::time::Duration;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use std::process::{Command, Stdio};
    /// use proc_guard::{ProcGuard, ProcessTermination};
    ///
    /// let child = if cfg!(target_os = "windows") {
    ///     Command::new("timeout").args(["/t", "2"]).spawn()?
    /// } else {
    ///     Command::new("sleep").arg("2").spawn()?
    /// };
    ///
    /// let guard = ProcGuard::new(child, ProcessTermination::CtrlCWait);
    ///
    /// // The child process will receive a CtrlC signal and will wait for its termination as soon as this function returns.
    /// // Note that it might cause issues to send a CtrlC signal if the process is not yet started.
    /// #
    /// #     thread::sleep(Duration::from_secs(1));
    /// #     Ok(())
    /// # }
    /// ```
    pub fn new(child: Child, termination: ProcessTermination) -> Self {
        ProcGuard {
            child: ManuallyDrop::new(child),
            termination: termination,
            dropped: false,
        }
    }

    /// Spawn the desired process into a new `ProcGuard`.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to create the child process to guard.
    /// * `termination` - The termination strategy to use when dropping the guard.
    ///
    /// # Returns
    ///
    /// * `ProcGuard` - A guard around the child process.
    ///
    /// # Example
    /// ```
    /// # use std::error::Error;
    /// # use std::thread;
    /// # use std::time::Duration;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use std::process::{Command, Stdio};
    /// use proc_guard::{ProcGuard, ProcessTermination};
    ///
    /// let guard = if cfg!(target_os = "windows") {
    ///     ProcGuard::spawn(
    ///         Command::new("timeout").args(["/t", "2"]),
    ///         ProcessTermination::CtrlCWait,
    ///     )?
    /// } else {
    ///     ProcGuard::spawn(
    ///         Command::new("sleep").arg("2"),
    ///         ProcessTermination::CtrlCWait,
    ///     )?
    /// };
    ///
    /// // The child process will receive a CtrlC signal and will wait for its termination as soon as this function returns.
    /// // Note that it might cause issues to send a CtrlC signal if the process is not yet started.
    /// #     thread::sleep(Duration::from_secs(1));
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    pub fn spawn(command: &mut Command, termination: ProcessTermination) -> io::Result<Self> {
        Ok(ProcGuard {
            child: ManuallyDrop::new(command.spawn()?),
            termination: termination,
            dropped: false,
        })
    }

    /// Returns a reference to the child process.
    ///
    /// # Example
    /// ```
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use std::process::{Command, Stdio};
    /// use proc_guard::{ProcGuard, ProcessTermination};
    ///
    /// let child = if cfg!(target_os = "windows") {
    ///     Command::new("timeout").args(["/t", "2"]).spawn()?
    /// } else {
    ///     Command::new("sleep").arg("2").spawn()?
    /// };
    ///
    /// let guard = ProcGuard::new(child, ProcessTermination::Wait);
    /// let child = guard.child();
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    pub fn child(&self) -> &Child {
        &self.child
    }

    /// Returns a mutable reference to the child process.
    ///
    /// # Example
    /// ```
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use std::process::{Command, Stdio};
    /// use proc_guard::{ProcGuard, ProcessTermination};
    ///
    /// let child = if cfg!(target_os = "windows") {
    ///     Command::new("timeout").args(["/t", "2"]).spawn()?
    /// } else {
    ///     Command::new("sleep").arg("2").spawn()?
    /// };
    ///
    /// let mut guard = ProcGuard::new(child, ProcessTermination::Wait);
    /// let child = guard.mut_child();
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    pub fn mut_child(&mut self) -> &mut Child {
        &mut self.child
    }

    /// Releases the guard without terminating the process.
    ///
    /// # Returns
    ///
    /// * `Child` - The child process that was guarded.
    ///
    /// # Example
    /// ```
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use std::process::{Command, Stdio};
    /// use proc_guard::{ProcGuard, ProcessTermination};
    ///
    /// let child = if cfg!(target_os = "windows") {
    ///     Command::new("timeout").args(["/t", "2"]).spawn()?
    /// } else {
    ///     Command::new("sleep").arg("2").spawn()?
    /// };
    ///
    /// let guard = ProcGuard::new(child, ProcessTermination::Wait);
    /// let child = guard.release();
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    pub fn release(mut self) -> Child {
        self.dropped = true;
        unsafe { ManuallyDrop::take(&mut self.child) }
    }

    /// Private implementation method to terminate the process.
    /// This method is called by the public `terminate` method and the `drop` method.
    ///
    /// # Note
    /// This method should not be called directly. Use the public `terminate` method instead.
    fn _drop_impl(&mut self) -> Result<Option<ExitStatus>, Error> {
        if self.dropped {
            // Indicates that the process was already terminated, it happens when drop is called after terminate
            return Ok(None);
        }
        self.dropped = true;

        terminate(&mut self.child, self.termination)
    }

    /// Terminates the process according to the specified termination strategy and releases the guard.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(ExitStatus))` - If the process terminates successfully in a graceful manner. In other word, a wait ended without timeout.
    /// * `Ok(None)` - If the process was forcefully killed.
    /// * `Err(Error)` - If an error occurs during termination.
    ///
    /// # Example
    /// ```
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use std::process::{Command, Stdio};
    /// use proc_guard::{ProcGuard, ProcessTermination};
    ///
    /// let child = if cfg!(target_os = "windows") {
    ///     Command::new("timeout").args(["/t", "2"]).spawn()?
    /// } else {
    ///     Command::new("sleep").arg("2").spawn()?
    /// };
    ///
    /// let guard = ProcGuard::new(child, ProcessTermination::Wait);
    /// let exit_status = guard.terminate();
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    pub fn terminate(mut self) -> Result<Option<ExitStatus>, Error> {
        self._drop_impl()
    }
}

/// Wait for a specified duration, then kill the process if it hasn't exited.
///
/// # Returns
///
/// * `Ok(Some(ExitStatus))` - If the wait did not timeout.
/// * `Ok(None)` - If the wait timeouted and kill was called.
/// * `Err(Error)` - If an error occurs during termination.
///
fn _wait_timeout_kill(child: &mut Child, timeout: Duration) -> Result<Option<ExitStatus>, Error> {
    match child.wait_timeout(timeout) {
        Ok(v) => Ok(Some(v)),
        Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
            child.kill()?;
            Ok(None)
        }
        Err(e) => Err(e.into()),
    }
}

/// Terminates the process according to the specified termination strategy.
///
/// # Arguments
///
/// * `child` - The child process to terminate.
/// * `termination` - The termination strategy to use.
///
/// # Returns
///
/// * `Ok(Some(ExitStatus))` - If the process terminates successfully in a graceful manner. In other word, a wait ended without timeout.
/// * `Ok(None)` - If the process was forcefully killed.
/// * `Err(Error)` - If an error occurs during termination.
///
/// # Errors
///
/// This function will return an error in the following situations:
///
/// * `proc_guard::Error::FailedToSendCtrlC` - with a system specific error code if the internal OS API failed.
///
/// * `proc_guard::Error::Timeout` - If the specified timeout duration elapses before the process completes AND the termination procedure did not attempt to kill the process afterward.
///
/// * `proc_guard::Error::WaitFailed` - with a system specific error code if the internal OS API failed.
///
/// * Other `proc_guard::Error` variants - For other errors that might occur while waiting for the process.
///
/// # Example
/// ```
/// # use std::error::Error;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use std::process::{Command, Stdio};
/// use proc_guard::{terminate, ProcessTermination};
///
/// let mut child = if cfg!(target_os = "windows") {
///     Command::new("timeout").args(["/t", "2"]).spawn()?
/// } else {
///     Command::new("sleep").arg("2").spawn()?
/// };
///
/// let result = terminate(&mut child, ProcessTermination::Wait);
/// #
/// #     Ok(())
/// # }
/// ```
pub fn terminate(
    child: &mut Child,
    termination: ProcessTermination,
) -> Result<Option<ExitStatus>, Error> {
    match termination {
        ProcessTermination::Wait => Ok(Some(child.wait()?)),
        ProcessTermination::WaitTimeout(timeout) => Ok(Some(child.wait_timeout(timeout)?)),
        ProcessTermination::WaitTimeoutKill(timeout) => _wait_timeout_kill(child, timeout),
        ProcessTermination::CtrlC => {
            send_ctrl_c(child)?;
            Ok(None)
        }
        ProcessTermination::CtrlCWait => {
            send_ctrl_c(child)?;
            Ok(Some(child.wait()?))
        }
        ProcessTermination::CtrlCWaitTimeout(timeout) => {
            send_ctrl_c(child)?;
            Ok(Some(child.wait_timeout(timeout)?))
        }
        ProcessTermination::CtrlCWaitTimeoutKill(timeout) => {
            send_ctrl_c(child)?;
            _wait_timeout_kill(child, timeout)
        }
        ProcessTermination::Kill => {
            child.kill()?;
            Ok(None)
        }
        ProcessTermination::KillWait => {
            child.kill()?;
            Ok(Some(child.wait()?))
        }
    }
}

impl Drop for ProcGuard {
    /// Ensures the process is terminated when the guard is dropped.
    fn drop(&mut self) {
        let _ = self._drop_impl();
    }
}
