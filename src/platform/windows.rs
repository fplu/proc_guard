use std::process::Child;

use winapi::um::errhandlingapi::GetLastError;
use winapi::um::wincon::{GenerateConsoleCtrlEvent, CTRL_C_EVENT};

use crate::error::Error;

pub(crate) fn _send_ctrl_c(child: &mut Child) -> Result<(), Error> {
    let result = unsafe { GenerateConsoleCtrlEvent(CTRL_C_EVENT, child.id()) };

    if result == 0 {
        let last_error = unsafe { GetLastError() };
        Err(Error::FailedToSendCtrlC(last_error))
    } else {
        Ok(())
    }
}
