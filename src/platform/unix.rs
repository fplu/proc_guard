use std::process::{Child, ExitStatus};
use std::thread;
use std::time::{Duration, Instant};

use libc::{kill, EIO, SIGINT};

use crate::error::Error;

pub fn _send_ctrl_c(child: &mut Child) -> Result<(), Error> {
    let pid = child.id() as i32;
    let result = unsafe { kill(pid, SIGINT) };

    if result != 0 {
        Err(Error::FailedToSendCtrlC(
            std::io::Error::last_os_error()
                .raw_os_error()
                .unwrap_or(EIO) as u32,
        ))
    } else {
        Ok(())
    }
}

pub fn _wait_timeout(child: &mut Child, timeout: Duration) -> Result<ExitStatus, Error> {
    let start = Instant::now();
    let sleep_duration = Duration::from_millis(10);

    while start.elapsed() < timeout {
        match child.try_wait() {
            Ok(Some(status)) => return Ok(status),
            Ok(None) => thread::sleep(sleep_duration),
            Err(_) => {
                return Err(Error::WaitFailed(
                    std::io::Error::last_os_error()
                        .raw_os_error()
                        .unwrap_or(EIO) as u32,
                ));
            }
        }
    }
    Err(Error::Timeout)
}
