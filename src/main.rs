use std::env;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::process;

use everdude::{
    from_unix_ms_i128, from_unix_ns_i128, from_unix_secs, from_unix_secs_i128,
    from_unix_us_i128, to_unix_secs,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_help() {
    let mut out = io::stdout().lock();
    let _ = writeln!(
        out,
        "everdude {VERSION} - Unix UTC <-> everdude half-week converter

USAGE:
    everdude [OPTIONS] [TIMESTAMP...]

OPTIONS:
    -r, --reverse    Convert everdude half-weeks (h) back to Unix seconds
    -h, --help       Print help information
    -V, --version    Print version information

INPUT:
    Reads timestamps from arguments, or from stdin (whitespace/line separated)
    when no timestamp arguments are provided.
    Forward mode auto-detects ns/us/ms by integer magnitude; ambiguous or
    fractional inputs default to seconds."
    );
}

#[inline]
fn parse_i128_ascii(bytes: &[u8]) -> Option<i128> {
    if bytes.is_empty() {
        return None;
    }
    let (neg, digits) = match bytes[0] {
        b'-' => (true, &bytes[1..]),
        b'+' => (false, &bytes[1..]),
        _ => (false, bytes),
    };
    if digits.is_empty() {
        return None;
    }
    let mut acc: u128 = 0;
    for &b in digits {
        if !b.is_ascii_digit() {
            return None;
        }
        acc = acc.checked_mul(10)?.checked_add((b - b'0') as u128)?;
    }
    if neg {
        if acc == (i128::MAX as u128) + 1 {
            Some(i128::MIN)
        } else if acc <= i128::MAX as u128 {
            Some(-(acc as i128))
        } else {
            None
        }
    } else if acc <= i128::MAX as u128 {
        Some(acc as i128)
    } else {
        None
    }
}

#[inline]
fn convert_token(token: &str, reverse: bool) -> Result<f64, ()> {
    if reverse {
        let h = token.parse::<f64>().map_err(|_| ())?;
        if !h.is_finite() {
            return Err(());
        }
        return Ok(to_unix_secs(h));
    }

    let bytes = token.as_bytes();
    if let Some(v) = parse_i128_ascii(bytes) {
        let abs = v.unsigned_abs();
        let h = if abs >= 100_000_000_000_000_000 {
            from_unix_ns_i128(v)
        } else if abs >= 100_000_000_000_000 {
            from_unix_us_i128(v)
        } else if abs >= 100_000_000_000 {
            from_unix_ms_i128(v)
        } else {
            from_unix_secs_i128(v)
        };
        return Ok(h);
    }

    let secs = token.parse::<f64>().map_err(|_| ())?;
    if !secs.is_finite() {
        return Err(());
    }
    Ok(from_unix_secs(secs))
}

#[inline]
fn write_value<W: Write>(out: &mut W, val: f64) -> io::Result<()> {
    writeln!(out, "{val}")
}

fn handle_io_err(err: io::Error) -> ! {
    if err.kind() == io::ErrorKind::BrokenPipe {
        process::exit(0);
    }
    let _ = writeln!(io::stderr().lock(), "everdude: I/O error: {err}");
    process::exit(1);
}

fn main() {
    let mut reverse = false;
    let mut end_of_flags = false;
    let mut inputs: Vec<String> = Vec::new();

    for arg in env::args().skip(1) {
        let bytes = arg.as_bytes();
        let is_flag = !end_of_flags
            && bytes.len() > 1
            && bytes[0] == b'-'
            && !bytes[1].is_ascii_digit()
            && bytes[1] != b'.';

        if is_flag {
            match arg.as_str() {
                "--" => end_of_flags = true,
                "-r" | "--reverse" => reverse = true,
                "-h" | "--help" => {
                    print_help();
                    return;
                }
                "-V" | "--version" => {
                    let _ = writeln!(io::stdout().lock(), "everdude {VERSION}");
                    return;
                }
                _ => {
                    let _ = writeln!(io::stderr().lock(), "everdude: unknown option '{arg}'");
                    process::exit(2);
                }
            }
        } else {
            inputs.push(arg);
        }
    }

    let stdout = io::stdout();
    let mut out = BufWriter::with_capacity(64 * 1024, stdout.lock());

    if !inputs.is_empty() {
        for token in &inputs {
            let trimmed = token.trim();
            if trimmed.is_empty() {
                continue;
            }
            match convert_token(trimmed, reverse) {
                Ok(val) => {
                    if let Err(e) = write_value(&mut out, val) {
                        handle_io_err(e);
                    }
                }
                Err(()) => {
                    let _ = out.flush();
                    let _ = writeln!(
                        io::stderr().lock(),
                        "everdude: invalid timestamp '{trimmed}'"
                    );
                    process::exit(1);
                }
            }
        }
    } else {
        let stdin = io::stdin();
        let mut reader = BufReader::with_capacity(64 * 1024, stdin.lock());
        let mut line = String::with_capacity(128);

        loop {
            line.clear();
            match reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    for token in line.split_ascii_whitespace() {
                        match convert_token(token, reverse) {
                            Ok(val) => {
                                if let Err(e) = write_value(&mut out, val) {
                                    handle_io_err(e);
                                }
                            }
                            Err(()) => {
                                let _ = out.flush();
                                let _ = writeln!(
                                    io::stderr().lock(),
                                    "everdude: invalid timestamp '{token}'"
                                );
                                process::exit(1);
                            }
                        }
                    }
                }
                Err(e) => handle_io_err(e),
            }
        }
    }

    if let Err(e) = out.flush() {
        handle_io_err(e);
    }
}
