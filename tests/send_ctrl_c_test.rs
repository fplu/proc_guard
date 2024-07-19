extern crate utilities;

#[cfg(test)]
mod tests {
    use proc_guard::send_ctrl_c;
    use std::thread;
    use std::time::Duration;
    use utilities;

    #[test]
    fn test_send_ctrl_c_unix() {
        // Spawn a long-running process
        let mut child = utilities::sleep_child("10");

        // Wait a little to avoid annoying windows pop-up
        thread::sleep(Duration::from_secs(1));

        // Send Ctrl-C signal to the process
        let result = send_ctrl_c(&mut child);

        // Verify that the signal was sent successfully
        assert!(result.is_ok());
    }
}
