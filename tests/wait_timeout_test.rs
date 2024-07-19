extern crate utilities;

#[cfg(test)]
mod tests {
    use proc_guard::wait_timeout;
    use proc_guard::Error;
    use std::time::Duration;
    use utilities;

    #[test]
    fn test_wait_timeout_success() {
        // Spawn a short-lived process
        let mut child = utilities::sleep_child("1");

        // Wait for the process to exit with a timeout
        let result = wait_timeout(&mut child, Duration::from_secs(5));

        // Verify that the process exited successfully
        assert!(result.is_ok());
    }

    #[test]
    fn test_wait_timeout_very_big_success() {
        // Spawn a short-lived process
        let mut child = utilities::sleep_child("1");

        // Wait for the process to exit with a timeout
        let result = wait_timeout(&mut child, Duration::from_secs(4_294_967_295u64));

        // Verify that the process exited successfully
        assert!(result.is_ok());
    }

    #[test]
    fn test_wait_timeout_exceeded() {
        // Spawn a long-running process
        let mut child = utilities::sleep_child("3");

        // Wait for the process to exit with a short timeout
        let result = wait_timeout(&mut child, Duration::from_secs(1));

        // Verify that the timeout was exceeded
        assert!(matches!(result, Err(Error::Timeout)));
    }
}
