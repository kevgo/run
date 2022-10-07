use crate::domain::{Stack, Stacks, Task};
use std::fmt::Display;
use std::path::Path;
use std::process::Command;

struct RustCargoStack {
    tasks: Vec<Task>,
}

impl Display for RustCargoStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Rust (Cargo)")
    }
}

impl Stack for RustCargoStack {
    fn setup(&self) -> Option<Command> {
        None
    }

    fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}

pub fn scan(stacks: &mut Stacks) {
    if !Path::new("Cargo.lock").exists() {
        return;
    }
    stacks.push(Box::new(RustCargoStack {
        tasks: vec![
            Task {
                name: "build".into(),
                cmd: "cargo".into(),
                argv: vec!["build".into()],
                desc: "cargo build".into(),
            },
            Task {
                name: "check".into(),
                cmd: "cargo".into(),
                argv: vec!["check".into()],
                desc: "cargo check".into(),
            },
            Task {
                name: "test".into(),
                cmd: "cargo".into(),
                argv: vec!["test".into()],
                desc: "cargo test".into(),
            },
        ],
    }));
}
