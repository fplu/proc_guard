/// # Process Guard Crate
///
/// Welcome to the `proc_guard` crate! This crate provides robust utilities for managing and terminating child processes in Rust applications. The primary components are the `ProcGuard` struct and the `ProcessTermination` enum, which together offer flexible and powerful strategies for handling process lifecycles.
///
/// ## Features
///
/// - **Process Guarding**: Safely manage the lifecycle of child processes with the `ProcGuard` struct, ensuring they are terminated according to specified strategies.
/// - **Termination Strategies**: Utilize the `ProcessTermination` enum to define various termination strategies, including waiting, timeouts, Ctrl+C signals, and immediate termination.
/// - **Error Handling**: Comprehensive error handling to manage process termination scenarios gracefully.
///
/// ## Termination Strategies
///
/// The `ProcessTermination` enum provides several strategies for terminating child processes:
/// - `Wait`: Wait indefinitely for the process to exit.
/// - `WaitTimeout(Duration)`: Wait for a specified duration for the process to exit.
/// - `WaitTimeoutKill(Duration)`: Wait for a specified duration, then kill the process if it hasn't exited.
/// - `CtrlC`: Send a Ctrl+C signal to the process and do not wait.
/// - `CtrlCWait`: Send a Ctrl+C signal and wait indefinitely for the process to exit.
/// - `CtrlCWaitTimeout(Duration)`: Send a Ctrl+C signal and wait for a specified duration for the process to exit.
/// - `CtrlCWaitTimeoutKill(Duration)`: Send a Ctrl+C signal, wait for a specified duration, then kill the process if it hasn't exited.
/// - `Kill`: Kill the process immediately and do not wait.
/// - `KillWait`: Kill the process immediately and wait indefinitely for the process to exit.
///
/// ## Examples
///
/// Here are some examples of how to use this crate:
///
/// ```rust
/// use std::process::{Command, Stdio};
/// use proc_guard::{ProcGuard, ProcessTermination};
///
/// let child = if cfg!(target_os = "windows") {
///     Command::new("timeout").args(["/t", "2"]).spawn()
/// } else {
///     Command::new("sleep").arg("2").spawn()
/// }.expect("Could not start command");
///
/// let guard = ProcGuard::new(child, ProcessTermination::Wait);
/// ```
///
/// ## Installation
///
/// Add this to your `Cargo.toml`:
///
/// ```toml
/// [dependencies]
/// proc_guard = "0.1.0"
/// ```
///
/// ## Usage
///
/// The primary structure is `ProcGuard`, which wraps around a `Child` process. When the `ProcGuard` is dropped, it ensures the child process is terminated according to the specified strategy.
///
/// ```rust
/// use std::process::{Command, Stdio};
/// use proc_guard::{ProcGuard, ProcessTermination};
///
/// let child = if cfg!(target_os = "windows") {
///     Command::new("timeout").args(["/t", "2"]).spawn()
/// } else {
///     Command::new("sleep").arg("2").spawn()
/// }.expect("Could not start command");
///
/// let guard = ProcGuard::new(child, ProcessTermination::Wait);
/// let exit_status = guard.terminate();
/// ```
///
/// ## Contributing
///
/// Contributions are welcome! Please open issues and submit pull requests on GitHub.
///
/// ## License
///
/// This crate is licensed under the MIT License. See the `LICENCE` file for more details.
mod error;
mod guard;
mod platform;

pub use error::*;
pub use guard::*;
pub use platform::*;
