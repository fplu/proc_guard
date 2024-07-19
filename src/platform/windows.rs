use std::os::windows::io::AsRawHandle;
use std::process::{Child, ExitStatus};
use std::time::Duration;

use winapi::shared::winerror::WAIT_TIMEOUT;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::winbase::WAIT_OBJECT_0;
use winapi::um::wincon::{GenerateConsoleCtrlEvent, CTRL_C_EVENT};

use crate::error::Error;

pub fn _send_ctrl_c(child: &mut Child) -> Result<(), Error> {
    let result = unsafe { GenerateConsoleCtrlEvent(CTRL_C_EVENT, child.id()) };

    if result == 0 {
        let last_error = unsafe { GetLastError() };
        Err(Error::FailedToSendCtrlC(last_error))
    } else {
        Ok(())
    }
}

fn _wait_timeout_impl(child: &mut Child, timeout_ms: u32) -> Result<ExitStatus, Error> {
    let handle = child.as_raw_handle();

    let winapi_handle: *mut winapi::ctypes::c_void = handle as *mut winapi::ctypes::c_void;

    let result = unsafe { WaitForSingleObject(winapi_handle, timeout_ms) };

    if result == WAIT_TIMEOUT {
        Err(Error::Timeout)
    } else if result == WAIT_OBJECT_0 {
        match child.wait() {
            Ok(v) => return Ok(v),
            Err(_) => return Err(Error::UnknownError()),
        }
    } else {
        let last_error = unsafe { GetLastError() };
        Err(Error::WaitFailed(last_error))
    }
}

pub fn _wait_timeout(child: &mut Child, timeout: Duration) -> Result<ExitStatus, Error> {
    const U32_MAX: u128 = u32::MAX as u128;
    let mut timeout_ms = timeout.as_millis();

    while timeout_ms > U32_MAX {
        match _wait_timeout_impl(child, u32::MAX) {
            Ok(v) => return Ok(v),
            Err(Error::Timeout) => {
                // nothing
            }
            Err(e) => return Err(e),
        };
        timeout_ms -= U32_MAX;
    }

    _wait_timeout_impl(child, timeout_ms as u32)
}
