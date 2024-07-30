# proc_guard

`proc_guard` is a Rust crate designed to manage and ensure the proper termination of child processes using various termination strategies. This crate provides a guard for a child process that ensures the process is terminated according to a specified strategy when the guard is dropped.

## Features

- Different termination strategies such as waiting, sending Ctrl+C, and killing the process.
- Ensures the proper cleanup of child processes when the guard goes out of scope.
- Supports both blocking and timeout-based termination methods.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
proc_guard = "0.2.0"
```

## Usage

Below are some examples to illustrate how to use the `ProcGuard` crate.

### Example 1: Basic Usage

```rust
use std::process::Command;
use proc_guard::{ProcGuard, ProcessTermination};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let child = if cfg!(target_os = "windows") {
        Command::new("timeout").args(["/t", "2"]).spawn()?
    } else {
        Command::new("sleep").arg("2").spawn()?
    };

    let guard = ProcGuard::new(child, ProcessTermination::Wait);
    // The child process will be waited upon until it exits.
    Ok(())
}
```

### Example 2: Using Ctrl+C and Wait

```rust
use std::process::Command;
use proc_guard::{ProcGuard, ProcessTermination};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let child = if cfg!(target_os = "windows") {
        Command::new("timeout").args(["/t", "2"]).spawn()?
    } else {
        Command::new("sleep").arg("2").spawn()?
    };

    let guard = ProcGuard::new(child, ProcessTermination::CtrlCWait);
    // The child process will receive a Ctrl+C signal and will be waited upon until it exits.
    Ok(())
}
```

### Example 3: Spawn a New Guarded Process

```rust
use std::process::Command;
use proc_guard::{ProcGuard, ProcessTermination};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let guard = if cfg!(target_os = "windows") {
        ProcGuard::spawn(Command::new("timeout").args(["/t", "2"]), ProcessTermination::CtrlCWait)?
    } else {
        ProcGuard::spawn(Command::new("sleep").arg("2"), ProcessTermination::CtrlCWait)?
    };
    // The child process will be managed and terminated as specified.
    Ok(())
}
```

### Example 4: Releasing the Guard

```rust
use std::process::Command;
use proc_guard::{ProcGuard, ProcessTermination};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let child = if cfg!(target_os = "windows") {
        Command::new("timeout").args(["/t", "2"]).spawn()?
    } else {
        Command::new("sleep").arg("2").spawn()?
    };

    let guard = ProcGuard::new(child, ProcessTermination::Wait);
    let child = guard.release();
    // The child process is now unmanaged and must be manually handled.
    Ok(())
}
```

## Termination Strategies

The `ProcessTermination` enum provides various strategies for terminating a process:

- `Wait`: Wait indefinitely for the process to exit.
- `WaitTimeout(Duration)`: Wait for a specified duration for the process to exit.
- `WaitTimeoutKill(Duration)`: Wait for a specified duration, then kill the process if it hasn't exited.
- `CtrlC`: Send a Ctrl+C signal to the process and does not wait.
- `CtrlCWait`: Send a Ctrl+C signal and wait indefinitely for the process to exit.
- `CtrlCWaitTimeout(Duration)`: Send a Ctrl+C signal and wait for a specified duration for the process to exit.
- `CtrlCWaitTimeoutKill(Duration)`: Send a Ctrl+C signal, wait for a specified duration, and then kill the process if it hasn't exited.
- `Kill`: Kill the process immediately and does not wait.
- `KillWait`: Kill the process immediately and wait indefinitely for the process to exit.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

