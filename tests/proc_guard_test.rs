#[cfg(test)]
mod tests {
    use proc_guard::{terminate, wait_timeout, ProcGuard, ProcessTermination};
    use std::thread;
    use std::time::Duration;
    use utilities;

    #[test]
    fn test_send_process_termination() {
        fn assert_send<T: Send>() {}
        assert_send::<ProcessTermination>();
    }

    #[test]
    fn test_sync_process_termination() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<ProcessTermination>();
    }

    #[test]
    fn test_send_proc_guard() {
        fn assert_send<T: Send>() {}
        assert_send::<ProcGuard>();
    }

    #[test]
    fn test_sync_proc_guard() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<ProcGuard>();
    }

    #[test]
    fn test_guard_wait() {
        let child = utilities::sleep_child("1");
        let guard = ProcGuard::new(child, ProcessTermination::Wait);
        assert!(guard.terminate().expect("Termination failed").is_some());
    }

    #[test]
    fn test_guard_wait2() {
        let mut command = utilities::sleep_command("1");
        let guard = ProcGuard::spawn(&mut command, ProcessTermination::Wait)
            .expect("Failed to start process");
        assert!(guard.terminate().expect("Termination failed").is_some());
    }

    #[test]
    fn test_guard_wait_timeout_no_timeout() {
        let child = utilities::sleep_child("1");
        let guard = ProcGuard::new(
            child,
            ProcessTermination::WaitTimeout(Duration::from_secs(2)),
        );
        assert!(guard.terminate().expect("Termination failed").is_some());
    }

    #[test]
    fn test_guard_wait_timeout_timeout() {
        let child = utilities::sleep_child("3");
        let guard = ProcGuard::new(
            child,
            ProcessTermination::WaitTimeout(Duration::from_secs(1)),
        );
        assert!(guard.terminate().is_err());
    }

    #[test]
    fn test_guard_wait_timeout_kill_no_timeout() {
        let child = utilities::sleep_child("1");
        let guard = ProcGuard::new(
            child,
            ProcessTermination::WaitTimeoutKill(Duration::from_secs(2)),
        );
        assert!(guard.terminate().expect("Termination failed").is_some());
    }

    #[test]
    fn test_guard_wait_timeout_kill_timeout() {
        let child = utilities::sleep_child("5");
        let guard = ProcGuard::new(
            child,
            ProcessTermination::WaitTimeoutKill(Duration::from_secs(1)),
        );
        let result = guard.terminate();
        assert!(result.is_ok());
        assert!(result.expect("Termination failed").is_none());
    }

    #[test]
    fn test_guard_ctrl_c() {
        let child = utilities::sleep_child("3");
        let guard = ProcGuard::new(child, ProcessTermination::CtrlC);
        thread::sleep(Duration::from_secs(1));
        assert!(guard.terminate().expect("Termination failed").is_none());
    }

    #[test]
    fn test_guard_ctrl_c_wait() {
        let child = utilities::sleep_child("2");
        let guard = ProcGuard::new(child, ProcessTermination::CtrlCWait);
        thread::sleep(Duration::from_secs(1));
        assert!(guard.terminate().expect("Termination failed").is_some());
    }

    #[test]
    fn test_guard_ctrl_c_wait_timeout_no_timeout() {
        let child = utilities::sleep_child("2");
        let guard = ProcGuard::new(
            child,
            ProcessTermination::CtrlCWaitTimeout(Duration::from_secs(5)),
        );
        thread::sleep(Duration::from_secs(1));
        assert!(guard.terminate().expect("Termination failed").is_some());
    }

    #[test]
    fn test_guard_ctrl_c_wait_timeout_timeout() {
        let child = utilities::sleep_child("5");
        let guard = ProcGuard::new(
            child,
            ProcessTermination::CtrlCWaitTimeout(Duration::from_secs(1)),
        );
        thread::sleep(Duration::from_secs(1));
        let result = guard.terminate();
        #[cfg(windows)]
        {
            assert!(result.is_err());
        }
        #[cfg(unix)]
        {
            assert!(result.is_ok());
            assert!(result.expect("Termination failed").is_some());
        }
    }

    #[test]
    fn test_guard_ctrl_c_wait_timeout_kill_no_timeout() {
        let child = utilities::sleep_child("1");
        let guard = ProcGuard::new(
            child,
            ProcessTermination::CtrlCWaitTimeoutKill(Duration::from_secs(5)),
        );
        thread::sleep(Duration::from_secs(1));
        assert!(guard.terminate().expect("Termination failed").is_some());
    }

    #[test]
    fn test_guard_ctrl_c_wait_timeout_kill_timeout() {
        let child = utilities::sleep_child("5");
        let guard = ProcGuard::new(
            child,
            ProcessTermination::CtrlCWaitTimeoutKill(Duration::from_secs(1)),
        );
        thread::sleep(Duration::from_secs(1));
        let result = guard.terminate();
        assert!(result.is_ok());
        #[cfg(windows)]
        {
            assert!(result.expect("Termination failed").is_none());
        }
        #[cfg(unix)]
        {
            assert!(result.expect("Termination failed").is_some());
        }
    }

    #[test]
    fn test_guard_kill() {
        let child = utilities::sleep_child("3");
        let guard = ProcGuard::new(child, ProcessTermination::Kill);
        assert!(guard.terminate().expect("Termination failed").is_none());
    }

    #[test]
    fn test_guard_kill_wait() {
        let child = utilities::sleep_child("3");
        let guard = ProcGuard::new(child, ProcessTermination::KillWait);
        assert!(guard.terminate().expect("Termination failed").is_some());
    }

    #[test]
    fn test_guard_child_getter() {
        let child = utilities::sleep_child("3");
        let guard = ProcGuard::new(child, ProcessTermination::Wait);

        // Use child to get a reference to the child process
        let child_ref = guard.child();

        // Ensure the PID is valid
        assert!(child_ref.id() > 0);
    }

    #[test]
    fn test_guard_mut_child_getter() {
        let child = utilities::sleep_child("3");
        let mut guard = ProcGuard::new(child, ProcessTermination::Wait);

        // Use mut_child to get a mutable reference to the child process
        let mut child_ref = guard.mut_child();

        // Ensure the PID is valid
        assert!(child_ref.id() > 0);
        // Ensure the process is still running
        let result = wait_timeout(&mut child_ref, Duration::from_secs(1));
        assert!(result.is_err()); // Should return a timeout error
    }

    #[test]
    fn test_guard_release() {
        let child = utilities::sleep_child("3");
        let guard = ProcGuard::new(child, ProcessTermination::Wait);

        // Release the guard without terminating the process
        let mut released_child = guard.release();

        // Ensure the PID is valid
        assert!(released_child.id() > 0);

        // Ensure the process is still running
        let result = wait_timeout(&mut released_child, Duration::from_secs(1));
        assert!(result.is_err()); // Should return a timeout error
    }

    #[test]
    fn test_wait() {
        let mut child = utilities::sleep_child("1");
        let result = terminate(&mut child, ProcessTermination::Wait);
        assert!(result.expect("Termination failed").is_some());
    }

    #[test]
    fn test_wait_timeout_no_timeout() {
        let mut child = utilities::sleep_child("1");
        let result = terminate(
            &mut child,
            ProcessTermination::WaitTimeout(Duration::from_secs(2)),
        );
        assert!(result.expect("Termination failed").is_some());
    }

    #[test]
    fn test_wait_timeout_timeout() {
        let mut child = utilities::sleep_child("3");
        let result = terminate(
            &mut child,
            ProcessTermination::WaitTimeout(Duration::from_secs(1)),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_wait_timeout_kill_no_timeout() {
        let mut child = utilities::sleep_child("1");
        let result = terminate(
            &mut child,
            ProcessTermination::WaitTimeoutKill(Duration::from_secs(2)),
        );
        assert!(result.expect("Termination failed").is_some());
    }

    #[test]
    fn test_wait_timeout_kill_timeout() {
        let mut child = utilities::sleep_child("3");
        let result = terminate(
            &mut child,
            ProcessTermination::WaitTimeoutKill(Duration::from_secs(1)),
        );
        assert!(result.expect("Termination failed").is_none());
    }

    #[test]
    fn test_ctrl_c() {
        let mut child = utilities::sleep_child("3");
        thread::sleep(Duration::from_secs(1));
        let result = terminate(&mut child, ProcessTermination::CtrlC);
        assert!(result.expect("Termination failed").is_none());
    }

    #[test]
    fn test_ctrl_c_wait() {
        let mut child = utilities::sleep_child("2");
        thread::sleep(Duration::from_secs(1));
        let result = terminate(&mut child, ProcessTermination::CtrlCWait);
        assert!(result.expect("Termination failed").is_some());
    }

    #[test]
    fn test_ctrl_c_wait_timeout_no_timeout() {
        let mut child = utilities::sleep_child("2");
        thread::sleep(Duration::from_secs(1));
        let result = terminate(
            &mut child,
            ProcessTermination::CtrlCWaitTimeout(Duration::from_secs(5)),
        );
        assert!(result.expect("Termination failed").is_some());
    }

    #[test]
    fn test_ctrl_c_wait_timeout_timeout() {
        let mut child = utilities::sleep_child("5");
        thread::sleep(Duration::from_secs(1));
        let result = terminate(
            &mut child,
            ProcessTermination::CtrlCWaitTimeout(Duration::from_secs(1)),
        );
        #[cfg(windows)]
        {
            assert!(result.is_err());
        }
        #[cfg(unix)]
        {
            assert!(result.is_ok());
            assert!(result.expect("Termination failed").is_some());
        }
    }

    #[test]
    fn test_ctrl_c_wait_timeout_kill_no_timeout() {
        let mut child = utilities::sleep_child("1");
        thread::sleep(Duration::from_secs(1));
        let result = terminate(
            &mut child,
            ProcessTermination::CtrlCWaitTimeoutKill(Duration::from_secs(5)),
        );
        assert!(result.expect("Termination failed").is_some());
    }

    #[test]
    fn test_ctrl_c_wait_timeout_kill_timeout() {
        let mut child = utilities::sleep_child("500");
        thread::sleep(Duration::from_secs(1));
        let result = terminate(
            &mut child,
            ProcessTermination::CtrlCWaitTimeoutKill(Duration::from_secs(1)),
        );
        assert!(result.is_ok());
        #[cfg(windows)]
        {
            assert!(result.expect("Termination failed").is_none());
        }
        #[cfg(unix)]
        {
            assert!(result.expect("Termination failed").is_some());
        }
    }

    #[test]
    fn test_kill() {
        let mut child = utilities::sleep_child("3");
        let result = terminate(&mut child, ProcessTermination::Kill);
        assert!(result.expect("Termination failed").is_none());
    }

    #[test]
    fn test_kill_wait() {
        let mut child = utilities::sleep_child("3");
        let result = terminate(&mut child, ProcessTermination::KillWait);
        assert!(result.expect("Termination failed").is_some());
    }
}
