use crate::{Stack, Stacks, Task};
use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, ErrorKind};
use std::path::Path;

use super::node_npm::PackageJson;

pub struct NodeYarnStack {
    tasks: Vec<Task>,
}

impl Display for NodeYarnStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Node.JS (yarn)")
    }
}

impl Stack for NodeYarnStack {
    fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}

pub fn scan(stacks: &mut Stacks) {
    if !Path::new("yarn.lock").exists() {
        return;
    }
    let file = match File::open("package.json") {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => return,
            e => {
                println!("Warning: Cannot read file \"package.json\": {}", e);
                return;
            }
        },
    };
    let reader = BufReader::new(file);
    let package_json: PackageJson = match serde_json::from_reader(reader) {
        Ok(content) => content,
        Err(e) => {
            println!(
                "Warning: file \"package.json\" has an invalid structure: {}",
                e
            );
            return;
        }
    };
    stacks.push(Box::new(NodeYarnStack {
        tasks: parse_scripts(package_json),
    }));
}

fn parse_scripts(package_json: PackageJson) -> Vec<Task> {
    let mut result = vec![];
    for (key, value) in package_json.scripts {
        result.push(Task {
            name: key.clone(),
            cmd: "yarn".into(),
            argv: vec!["--silent".into(), "run".into(), key],
            desc: Some(value),
        });
    }
    result.sort();
    result
}
