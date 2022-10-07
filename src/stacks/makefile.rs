use crate::domain::{Stack, Stacks, Task};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt::Display;
use std::fs;
use std::io::ErrorKind;
use std::process::Command;

struct MakefileStack {
    tasks: Vec<Task>,
}

impl Display for MakefileStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Makefile")
    }
}

impl Stack for MakefileStack {
    fn setup(&self) -> Option<Command> {
        None
    }

    fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    fn update(&self) -> Option<Command> {
        None
    }
}

pub fn scan(stacks: &mut Stacks) {
    let text = match fs::read_to_string("Makefile") {
        Ok(text) => text,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => return,
            e => {
                println!("Warning: Cannot read file \"Makefile\": {}", e);
                return;
            }
        },
    };
    stacks.push(Box::new(MakefileStack {
        tasks: parse_text(&text),
    }));
}

/// provides the tasks in the given Makefile content
fn parse_text(text: &str) -> Vec<Task> {
    let mut result = vec![];
    for line in text.lines() {
        if let Some(task) = parse_line(line) {
            result.push(task);
        }
    }
    result
}

/// provides a task for the Makefile target defined on the given line, if one exists
fn parse_line(line: &str) -> Option<Task> {
    let capture = match RE.captures(line) {
        Some(capture) => capture,
        None => return None,
    };
    let name = capture.get(1).unwrap().as_str();
    let desc = match capture.get(4) {
        Some(desc) => desc.as_str().to_string(),
        None => "".to_string(),
    };
    Some(Task {
        name: name.into(),
        cmd: "make".into(),
        argv: vec!["--no-print-directory".into(), name.into()],
        desc,
    })
}
static RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^([[[:alnum:]]-]+):([^#]*)?(#[[:blank:]]*(.*))?"#).unwrap());

#[cfg(test)]
mod tests {

    mod parse_line {
        use crate::domain::Task;

        #[test]
        fn no_task() {
            let give = "\techo hello";
            let want = None;
            let have = super::super::parse_line(give);
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn name() {
            let give = "cuke:";
            let want = Some(Task {
                name: "cuke".into(),
                cmd: "make".into(),
                argv: vec!["--no-print-directory".into(), "cuke".into()],
                desc: "".into(),
            });
            let have = super::super::parse_line(give);
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn name_and_deps() {
            let give = "cuke: build, lint";
            let want = Some(Task {
                name: "cuke".into(),
                cmd: "make".into(),
                argv: vec!["--no-print-directory".into(), "cuke".into()],
                desc: "".into(),
            });
            let have = super::super::parse_line(give);
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn name_and_desc() {
            let give = "cuke: # run cucumber";
            let want = Some(Task {
                name: "cuke".into(),
                cmd: "make".into(),
                argv: vec!["--no-print-directory".into(), "cuke".into()],
                desc: "run cucumber".into(),
            });
            let have = super::super::parse_line(give);
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn name_and_deps_and_desc() {
            let give = "cuke: build, lint # run cucumber";
            let want = Some(Task {
                name: "cuke".into(),
                cmd: "make".into(),
                argv: vec!["--no-print-directory".into(), "cuke".into()],
                desc: "run cucumber".into(),
            });
            let have = super::super::parse_line(give);
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn name_with_dash() {
            let give = "cuke-this:  # run only the tagged Cucumber scenario";
            let want = Some(Task {
                name: "cuke-this".into(),
                cmd: "make".into(),
                argv: vec!["--no-print-directory".into(), "cuke-this".into()],
                desc: "run only the tagged Cucumber scenario".into(),
            });
            let have = super::super::parse_line(give);
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn name_with_number() {
            let give = "task-1:  # task 1";
            let want = Some(Task {
                name: "task-1".into(),
                cmd: "make".into(),
                argv: vec!["--no-print-directory".into(), "task-1".into()],
                desc: "task 1".into(),
            });
            let have = super::super::parse_line(give);
            pretty::assert_eq!(have, want);
        }
    }
}
