use std::{
    fs,
    process::Command,
    time::{Duration, Instant},
};

use super::{Limits, run_bounded};

fn limits(timeout: Duration, bytes: usize) -> Limits {
    Limits {
        timeout,
        stdout_bytes: bytes,
        stderr_bytes: bytes,
    }
}

#[test]
fn captures_bounded_output_and_exit_status() {
    let output = run_bounded(
        Command::new("/bin/sh")
            .arg("-c")
            .arg("printf out; printf err >&2; exit 7"),
        "fixture",
        limits(Duration::from_secs(2), 64),
    )
    .unwrap();
    assert_eq!(output.status.code(), Some(7));
    assert_eq!(output.stdout, b"out");
    assert_eq!(output.stderr, b"err");
}

#[test]
fn kills_a_child_when_either_output_stream_exceeds_its_limit() {
    for script in [
        "while :; do printf 1234567890; done",
        "while :; do printf 1234567890 >&2; done",
    ] {
        let started = Instant::now();
        let error = run_bounded(
            Command::new("/bin/sh").arg("-c").arg(script),
            "noisy fixture",
            limits(Duration::from_secs(5), 1_024),
        )
        .unwrap_err();
        assert_eq!(error.code(), "COMMAND_OUTPUT_LIMIT");
        assert!(started.elapsed() < Duration::from_secs(2));
    }
}

#[test]
fn finite_fast_overflow_is_rejected() {
    let error = run_bounded(
        Command::new("/bin/sh").arg("-c").arg("printf 12345"),
        "finite noisy fixture",
        limits(Duration::from_secs(2), 4),
    )
    .unwrap_err();
    assert_eq!(error.code(), "COMMAND_OUTPUT_LIMIT");
}

#[test]
fn output_overflow_kills_the_entire_process_group() {
    let directory = tempfile_directory();
    let marker = directory.join("leaked");
    let script = format!(
        "(sleep 1; printf leaked > '{}') & while :; do printf 1234567890; done",
        marker.display()
    );
    let error = run_bounded(
        Command::new("/bin/sh").arg("-c").arg(script),
        "noisy tree fixture",
        limits(Duration::from_secs(2), 1_024),
    )
    .unwrap_err();
    assert_eq!(error.code(), "COMMAND_OUTPUT_LIMIT");
    assert_grandchild_did_not_run(&directory, &marker);
}

#[test]
fn timeout_kills_the_entire_process_group() {
    let directory = tempfile_directory();
    let marker = directory.join("leaked");
    let script = format!(
        "(sleep 1; printf leaked > '{}') & sleep 30",
        marker.display()
    );
    let error = run_bounded(
        Command::new("/bin/sh").arg("-c").arg(script),
        "tree fixture",
        limits(Duration::from_millis(100), 1_024),
    )
    .unwrap_err();
    assert_eq!(error.code(), "COMMAND_TIMEOUT");
    assert_grandchild_did_not_run(&directory, &marker);
}

fn assert_grandchild_did_not_run(directory: &std::path::Path, marker: &std::path::Path) {
    std::thread::sleep(Duration::from_millis(1_200));
    assert!(!marker.exists(), "a grandchild survived process-group kill");
    fs::remove_dir(directory).unwrap();
}

fn tempfile_directory() -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "bullet-process-test-{}-{}",
        std::process::id(),
        std::thread::current().name().unwrap_or("unnamed")
    ));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir(&path).unwrap();
    path
}

#[test]
fn timeout_is_bounded() {
    let started = Instant::now();
    let error = run_bounded(
        Command::new("/bin/sh").arg("-c").arg("sleep 30"),
        "slow fixture",
        limits(Duration::from_millis(100), 1_024),
    )
    .unwrap_err();
    assert_eq!(error.code(), "COMMAND_TIMEOUT");
    assert!(started.elapsed() < Duration::from_secs(2));
}

#[test]
fn exact_limit_is_allowed() {
    let output = run_bounded(
        Command::new("/bin/sh").arg("-c").arg("printf 1234"),
        "exact fixture",
        limits(Duration::from_secs(2), 4),
    )
    .unwrap();
    assert_eq!(output.stdout, b"1234");
}
