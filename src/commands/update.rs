use crate::domain::{Outcome, Workspace};
use std::process::Stdio;

pub fn update(workspace: Workspace) -> Outcome {
    let mut executed = false;
    for stack in workspace.stacks {
        let mut cmd = match stack.update() {
            Some(cmd) => cmd,
            None => continue,
        };
        executed = true;
        let output = cmd
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output();
        let output = match output {
            Ok(output) => output,
            Err(e) => return Outcome::CannotFindExecutable { err: e.to_string() },
        };
        if let Some(exit_code) = output.status.code() {
            if exit_code != 0 {
                return Outcome::ScriptFailed {
                    exit_code: exit_code as u8,
                };
            }
        };
    }
    if executed {
        Outcome::Success
    } else {
        Outcome::NoUpdate
    }
}
