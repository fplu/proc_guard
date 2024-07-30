use std::process::Child;

use libc::{kill, pid_t, EIO, SIGINT};

use crate::error::Error;

pub(crate) fn _send_ctrl_c(child: &mut Child) -> Result<(), Error> {
    let pid = child.id() as pid_t;
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
