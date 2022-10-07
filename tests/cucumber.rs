use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};
use itertools::Itertools;
use rand::Rng;
use std::borrow::Cow;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};
use tokio::process::Command;

#[derive(Debug, World)]
#[world(init = Self::new)]
struct RunWorld {
    /// the directory containing the test files of the current scenario
    dir: PathBuf,

    /// the result of running Atlanta
    output: Option<Output>,
}

impl RunWorld {
    fn new() -> Self {
        Self {
            dir: tmp_dir(),
            output: None,
        }
    }
}

impl RunWorld {
    /// provides the exit code of the Atlanta run
    fn exit_code(&self) -> i32 {
        match &self.output {
            Some(output) => output.status.code().unwrap(),
            None => panic!(),
        }
    }

    /// provides the textual output of the Atlanta run
    fn output(&self) -> Cow<str> {
        match &self.output {
            Some(output) => String::from_utf8_lossy(&output.stdout),
            None => Default::default(),
        }
    }

    /// provides the textual output of the Atlanta run with whitespace trimmed from every line
    fn output_trimmed(&self) -> String {
        self.output()
            .trim()
            .lines()
            .map(|line| line.trim_end())
            .join("\n")
    }
}

#[given(expr = "a file {string} with content:")]
async fn a_file_with_content(
    world: &mut RunWorld,
    step: &Step,
    filename: String,
) -> io::Result<()> {
    let content = step.docstring.as_ref().unwrap().trim();
    create_file(&filename, content, &world.dir).await
}

#[given(expr = "a file {string}")]
async fn a_file(world: &mut RunWorld, filename: String) -> io::Result<()> {
    create_file(&filename, "", &world.dir).await
}

#[given("a Makefile with content:")]
async fn a_makefile(world: &mut RunWorld, step: &Step) -> io::Result<()> {
    let content = step.docstring.as_ref().unwrap().trim();
    let tabulized = convert_to_makefile_format(content);
    create_file("Makefile", &tabulized, &world.dir).await
}

#[when(expr = "executing {string}")]
async fn executing(world: &mut RunWorld, command: String) {
    let mut argv = command.split_ascii_whitespace();
    match argv.next() {
        Some("a") => {}
        _ => panic!("The end-to-end tests can only run the 'a' command for now"),
    }
    world.output = Some(
        Command::new("../../target/debug/a")
            .args(argv)
            .current_dir(&world.dir)
            .output()
            .await
            .expect("cannot find the 'a' executable"),
    );
}

#[then("it prints:")]
fn verify_output(world: &mut RunWorld, step: &Step) {
    let want = step.docstring.as_ref().unwrap().trim();
    let stripped = strip_ansi_escapes::strip(world.output_trimmed()).unwrap();
    let have = str::from_utf8(&stripped).unwrap();
    pretty::assert_eq!(have, want);
}

#[then(expr = "the exit code is {int}")]
fn exit_code(world: &mut RunWorld, want: i32) {
    assert_eq!(world.exit_code(), want);
}

#[then(expr = "the output contains {string}")]
fn output_contains(world: &mut RunWorld, text: String) {
    assert!(world.output().contains(&text));
}

#[then(expr = "the workspace contains a folder {string}")]
fn contains_folder(world: &mut RunWorld, folder: String) {
    assert!(world.dir.join(folder).is_dir())
}

/// creates a temporary directory
fn tmp_dir() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let rand: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(3)
        .map(char::from)
        .collect();
    let cwd = env::current_dir().expect("cannot determine the current directory");
    let dir = cwd.join("tmp").join(format!("{}-{}", timestamp, rand));
    fs::create_dir_all(&dir).unwrap();
    dir
}

async fn create_file(filename: &str, content: &str, dir: &Path) -> io::Result<()> {
    let filepath = dir.join(filename);
    let mut file = File::create(filepath).await?;
    file.write_all(content.as_bytes()).await?;
    file.flush().await
}

/// this codebase uses 2 spaces for indentation but Makefiles require tabs
fn convert_to_makefile_format(text: &str) -> String {
    let mut result = String::new();
    for line in text.lines() {
        if line.starts_with("  ") {
            result.push('\t');
            result.push_str(&line[2..]);
            result.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }
    result
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    RunWorld::run("features").await;
}
