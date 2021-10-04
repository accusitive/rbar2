use std::process::Command;

pub fn exec(cmd: &str) -> Option<String> {
    let stdout = Command::new("/bin/bash")
        .arg("-c")
        .arg(cmd)
        .output().ok()?
        .stdout;
    Some(std::str::from_utf8(&stdout).ok()?.to_string())
}
