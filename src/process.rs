//! Bounded child-process execution for local authority and installation tools.

use std::{
    io::{self, Read},
    process::{Command, Output, Stdio},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

#[cfg(unix)]
use std::os::unix::process::CommandExt;

use crate::coord::CoordError;

const POLL_INTERVAL: Duration = Duration::from_millis(10);

#[derive(Clone, Copy)]
pub(crate) struct Limits {
    pub(crate) timeout: Duration,
    pub(crate) stdout_bytes: usize,
    pub(crate) stderr_bytes: usize,
}

pub(crate) fn run_bounded(
    command: &mut Command,
    label: &str,
    limits: Limits,
) -> Result<Output, CoordError> {
    let deadline = Instant::now().checked_add(limits.timeout).ok_or_else(|| {
        CoordError::new("INVALID_COMMAND_DEADLINE", "command deadline overflowed")
    })?;
    command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(unix)]
    command.process_group(0);

    let mut child = command.spawn().map_err(|error| {
        CoordError::new(
            "COMMAND_START_FAILED",
            format!("could not start {label}: {error}"),
        )
    })?;
    let Some(stdout) = child.stdout.take() else {
        terminate_process_group(&mut child);
        let _ = child.wait();
        return Err(CoordError::new(
            "COMMAND_IO_FAILED",
            format!("{label} stdout pipe was unavailable"),
        ));
    };
    let Some(stderr) = child.stderr.take() else {
        terminate_process_group(&mut child);
        let _ = child.wait();
        return Err(CoordError::new(
            "COMMAND_IO_FAILED",
            format!("{label} stderr pipe was unavailable"),
        ));
    };
    let stdout_exceeded = Arc::new(AtomicBool::new(false));
    let stderr_exceeded = Arc::new(AtomicBool::new(false));
    let stdout_reader = match capture(stdout, limits.stdout_bytes, Arc::clone(&stdout_exceeded)) {
        Ok(reader) => reader,
        Err(error) => {
            terminate_process_group(&mut child);
            let _ = child.wait();
            return Err(CoordError::new(
                "COMMAND_IO_FAILED",
                format!("could not start {label} stdout reader: {error}"),
            ));
        }
    };
    let stderr_reader = match capture(stderr, limits.stderr_bytes, Arc::clone(&stderr_exceeded)) {
        Ok(reader) => reader,
        Err(error) => {
            terminate_process_group(&mut child);
            let _ = child.wait();
            let _ = join_capture(stdout_reader, label, "stdout");
            return Err(CoordError::new(
                "COMMAND_IO_FAILED",
                format!("could not start {label} stderr reader: {error}"),
            ));
        }
    };
    let status = loop {
        if stdout_exceeded.load(Ordering::Acquire) || stderr_exceeded.load(Ordering::Acquire) {
            terminate_process_group(&mut child);
            let _ = child.wait();
            drop(stdout_reader);
            drop(stderr_reader);
            return Err(output_limit_error(label, limits));
        }
        if Instant::now() >= deadline {
            terminate_process_group(&mut child);
            let _ = child.wait();
            drop(stdout_reader);
            drop(stderr_reader);
            return Err(CoordError::new(
                "COMMAND_TIMEOUT",
                format!(
                    "{label} exceeded its {} second deadline",
                    limits.timeout.as_secs()
                ),
            ));
        }
        if stdout_reader.is_finished() && stderr_reader.is_finished() {
            if stdout_exceeded.load(Ordering::Acquire) || stderr_exceeded.load(Ordering::Acquire) {
                terminate_process_group(&mut child);
                let _ = child.wait();
                let _ = join_capture(stdout_reader, label, "stdout");
                let _ = join_capture(stderr_reader, label, "stderr");
                return Err(output_limit_error(label, limits));
            }
            if Instant::now() >= deadline {
                terminate_process_group(&mut child);
                let _ = child.wait();
                let _ = join_capture(stdout_reader, label, "stdout");
                let _ = join_capture(stderr_reader, label, "stderr");
                return Err(CoordError::new(
                    "COMMAND_TIMEOUT",
                    format!(
                        "{label} exceeded its {} second deadline",
                        limits.timeout.as_secs()
                    ),
                ));
            }
            match child.try_wait() {
                Ok(Some(status)) => break status,
                Ok(None) => {}
                Err(error) => {
                    terminate_process_group(&mut child);
                    let _ = child.wait();
                    let _ = join_capture(stdout_reader, label, "stdout");
                    let _ = join_capture(stderr_reader, label, "stderr");
                    return Err(CoordError::new(
                        "COMMAND_WAIT_FAILED",
                        format!("could not wait for {label}: {error}"),
                    ));
                }
            }
        }
        thread::sleep(POLL_INTERVAL);
    };
    let stdout = join_capture(stdout_reader, label, "stdout");
    let stderr = join_capture(stderr_reader, label, "stderr");
    if stdout_exceeded.load(Ordering::Acquire) || stderr_exceeded.load(Ordering::Acquire) {
        return Err(output_limit_error(label, limits));
    }
    let stdout = stdout?;
    let stderr = stderr?;
    Ok(Output {
        status,
        stdout,
        stderr,
    })
}

fn output_limit_error(label: &str, limits: Limits) -> CoordError {
    CoordError::new(
        "COMMAND_OUTPUT_LIMIT",
        format!(
            "{label} exceeded its output limit (stdout {} bytes, stderr {} bytes)",
            limits.stdout_bytes, limits.stderr_bytes
        ),
    )
}

fn capture<R: Read + Send + 'static>(
    mut reader: R,
    limit: usize,
    exceeded: Arc<AtomicBool>,
) -> io::Result<JoinHandle<io::Result<Vec<u8>>>> {
    thread::Builder::new()
        .name("bullet-child-output".to_owned())
        .spawn(move || {
            let mut bytes = Vec::with_capacity(limit.min(8 * 1024));
            let mut buffer = [0_u8; 8 * 1024];
            loop {
                let count = reader.read(&mut buffer)?;
                if count == 0 {
                    return Ok(bytes);
                }
                let remaining = limit.saturating_sub(bytes.len());
                if count > remaining {
                    bytes.extend_from_slice(&buffer[..remaining]);
                    exceeded.store(true, Ordering::Release);
                    return Ok(bytes);
                }
                bytes.extend_from_slice(&buffer[..count]);
            }
        })
}

fn join_capture(
    reader: JoinHandle<io::Result<Vec<u8>>>,
    label: &str,
    stream: &str,
) -> Result<Vec<u8>, CoordError> {
    reader
        .join()
        .map_err(|_| {
            CoordError::new(
                "COMMAND_IO_FAILED",
                format!("{label} {stream} reader panicked"),
            )
        })?
        .map_err(|error| {
            CoordError::new(
                "COMMAND_IO_FAILED",
                format!("could not read {label} {stream}: {error}"),
            )
        })
}

#[cfg(unix)]
fn terminate_process_group(child: &mut std::process::Child) {
    use nix::{
        sys::signal::{Signal, killpg},
        unistd::Pid,
    };

    if let Ok(raw_pid) = i32::try_from(child.id()) {
        let _ = killpg(Pid::from_raw(raw_pid), Signal::SIGKILL);
    }
    let _ = child.kill();
}

#[cfg(not(unix))]
fn terminate_process_group(child: &mut std::process::Child) {
    let _ = child.kill();
}

#[cfg(all(test, unix))]
#[path = "../tests/support/process_unit.rs"]
mod tests;
