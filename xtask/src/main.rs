use std::env;
use std::process::{Command, ExitCode};

/// Supported `cargo xtask` subcommands.
///
/// This enum exists to:
/// - Avoid stringly-typed control flow.
/// - Let the compiler enforce exhaustive handling when new commands are added.
/// - Centralize the command vocabulary for the project.
#[derive(Debug, Clone, Copy)]
enum XTaskCmd {
    /// Run `cargo test`.
    Test,

    /// Run `cargo test` first, then `cargo build` if tests pass.
    Build,

    /// Run `cargo test` first, then `cargo run` if tests pass.
    Run,
}

/// Entry point for the `xtask` helper binary.
///
/// Responsibilities:
/// - Parse the xtask subcommand.
/// - Enforce the rule: **no build or run unless tests pass**.
/// - Delegate actual work to Cargo as child processes.
///
/// This binary intentionally contains *no project logic*.
/// It is only orchestration.
fn main() -> ExitCode {
    // Skip executable name and read the xtask subcommand.
    let mut args = env::args().skip(1);

    let Some(cmd_str) = args.next() else {
        print_usage();
        return ExitCode::from(2);
    };

    // Convert user input into a strongly-typed command.
    let Some(cmd) = parse_cmd(&cmd_str) else {
        eprintln!("unknown xtask command: {}", &cmd_str);
        print_usage();
        return ExitCode::from(2);
    };

    match cmd {
        // `cargo xtask test`
        XTaskCmd::Test => run_cargo(&["test"]),

        // `cargo xtask build`
        // Contract: build is only allowed if tests pass.
        XTaskCmd::Build => {
            if !run_cargo_ok(&["test"]) {
                return ExitCode::from(1);
            }
            run_cargo(&["build"])
        }

        // `cargo xtask run`
        // Contract: run is only allowed if tests pass.
        XTaskCmd::Run => {
            if !run_cargo_ok(&["test"]) {
                return ExitCode::from(1);
            }

            // Forward everything after `--` directly to `cargo run`.
            // Example:
            //   cargo xtask run -- --release
            let forwarded: Vec<String> = args.collect();
            let mut cargo_args = vec!["run".to_string()];
            cargo_args.extend(forwarded);

            run_cargo_owned(&cargo_args)
        }
    }
}

/// Print user-facing usage instructions.
///
/// Kept intentionally small and explicit:
/// this is a developer tool, not a CLI framework.
fn print_usage() {
    eprintln!("usage: cargo xtask <test|build|run> [-- <args forwarded to cargo>]");
}

/// Parse a raw string into an `XTaskCmd`.
///
/// This function is the *only* place where string matching is allowed.
/// Everywhere else operates on the enum for safety and clarity.
fn parse_cmd(s: &str) -> Option<XTaskCmd> {
    match s {
        "test" => Some(XTaskCmd::Test),
        "build" => Some(XTaskCmd::Build),
        "run" => Some(XTaskCmd::Run),
        _ => None,
    }
}

/// Run a Cargo command and return `true` if it succeeded.
///
/// This is intentionally a thin wrapper:
/// higher-level policy (what to do on failure) lives at the call site.
fn run_cargo_ok(args: &[&str]) -> bool {
    run_cargo(args) == ExitCode::SUCCESS
}

/// Run a Cargo command with borrowed arguments.
///
/// Responsibilities:
/// - Spawn `cargo` as a child process.
/// - Print the command being executed (debug visibility).
/// - Propagate Cargo's exit status faithfully.
fn run_cargo(args: &[&str]) -> ExitCode {
    let mut cmd = Command::new("cargo");
    cmd.args(args);

    eprintln!("> cargo {}", args.join(" "));
    match cmd.status() {
        Ok(status) => status
            .code()
            .map(|c| ExitCode::from(c as u8))
            .unwrap_or(ExitCode::from(1)),
        Err(err) => {
            eprintln!("failed to run cargo: {err}");
            ExitCode::from(1)
        }
    }
}

/// Same as `run_cargo`, but accepts owned arguments.
///
/// This exists to support argument forwarding (e.g. `--release`)
/// without unnecessary string slicing or lifetime juggling.
fn run_cargo_owned(args: &[String]) -> ExitCode {
    let mut cmd = Command::new("cargo");
    cmd.args(args);

    eprintln!("> cargo {}", args.join(" "));
    match cmd.status() {
        Ok(status) => status
            .code()
            .map(|c| ExitCode::from(c as u8))
            .unwrap_or(ExitCode::from(1)),
        Err(err) => {
            eprintln!("failed to run cargo: {err}");
            ExitCode::from(1)
        }
    }
}
