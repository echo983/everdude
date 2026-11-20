use std::io::Write;
use std::process::{Command, Stdio};

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_everdude")
}

#[test]
fn cli_positional_args_and_units() {
    let out = Command::new(bin())
        .args([
            "978307200",
            "978307200000",
            "978307200000000",
            "978307200000000000",
            "978609600",
            "978609600.0",
        ])
        .output()
        .expect("failed to execute binary");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(stdout, "0\n0\n0\n0\n1\n1\n");
}

#[test]
fn cli_negative_timestamps_without_double_dash() {
    let out = Command::new(bin())
        .args(["-r", "-1"])
        .output()
        .expect("failed to execute binary");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(stdout, "978004800\n");

    let out_fwd = Command::new(bin())
        .args(["-302400"])
        .output()
        .expect("failed to execute binary");
    assert!(out_fwd.status.success());
    let expected = format!("{}\n", (-302_400i64 - 978_307_200) as f64 / 302_400.0);
    assert_eq!(String::from_utf8_lossy(&out_fwd.stdout), expected);
}

#[test]
fn cli_stdin_pipe() {
    let mut child = Command::new(bin())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn binary");

    {
        let stdin = child.stdin.as_mut().unwrap();
        stdin
            .write_all(b"978307200\n\n  978609600   978912000 \n")
            .unwrap();
    }

    let out = child.wait_with_output().unwrap();
    assert!(out.status.success());
    assert_eq!(String::from_utf8_lossy(&out.stdout), "0\n1\n2\n");
}

#[test]
fn cli_reverse_stdin() {
    let mut child = Command::new(bin())
        .arg("-r")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn binary");

    {
        let stdin = child.stdin.as_mut().unwrap();
        stdin.write_all(b"0\n1\n2\n-1\n").unwrap();
    }

    let out = child.wait_with_output().unwrap();
    assert!(out.status.success());
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        "978307200\n978609600\n978912000\n978004800\n"
    );
}

#[test]
fn cli_invalid_input_exits_nonzero() {
    let out = Command::new(bin())
        .arg("not-a-number")
        .output()
        .expect("failed to execute binary");

    assert!(!out.status.success());
    assert!(!out.stderr.is_empty());
}
