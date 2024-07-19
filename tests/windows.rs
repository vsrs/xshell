#![cfg(windows)]

use xshell::{cmd, Shell};

#[test]
fn npm() {
    let sh = Shell::new().unwrap();

    if cmd!(sh, "where npm").read().is_ok() {
        let script_shell = cmd!(sh, "npm get shell").read().unwrap();
        assert!(script_shell.ends_with(".exe"));

        let script_shell_explicit = cmd!(sh, "npm.cmd get shell").read().unwrap();
        assert_eq!(script_shell, script_shell_explicit);
    }
}

#[test]
fn overridden_cmd_path() {
    let sh = Shell::new().unwrap();

    if cmd!(sh, "where npm").read().is_ok() {
        // should fail as `PATH` is overridden via Cmd
        assert!(cmd!(sh, "npm get shell").env("PATH", ".").run().is_err());
    }
}

#[test]
fn overridden_shell_path() {
    let mut sh = Shell::new().unwrap();
    sh.set_var("PATH", ".");

    if cmd!(sh, "where npm").read().is_ok() {
        // should fail as `PATH` is overridden via Shell
        assert!(cmd!(sh, "npm get shell").env("PATH", ".").run().is_err());
    }
}
