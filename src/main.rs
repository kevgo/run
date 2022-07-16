use domain::{Outcome, Workspace};
use std::env::Args;

mod commands;
mod domain;
mod stacks;
mod strings;

/// all CLI commands that could be run
enum Command {
    List,
    Run(String),
    Setup,
    FishCompletionSetup,
    FishCompletion,
}

fn parse_cli_args(mut args: Args) -> Command {
    args.next(); // skip the binary name
    match args.next() {
        Some(cmd) if cmd == "-s" => Command::Setup,
        Some(cmd) if cmd == "--setup" => Command::Setup,
        Some(cmd) if cmd == "--fish-completion" => Command::FishCompletion,
        Some(cmd) if cmd == "--print-fish-completions" => Command::FishCompletionSetup,
        Some(cmd) => Command::Run(cmd),
        None => Command::List,
    }
}

fn main() -> Outcome {
    let command = parse_cli_args(std::env::args());
    let outcome = execute(command);
    flatten(outcome)
}

fn execute(command: Command) -> Result<Outcome, Outcome> {
    Ok(match command {
        Command::List => commands::list(load_workspace()?),
        Command::Run(name) => commands::run(load_workspace()?, name),
        Command::Setup => commands::setup(load_workspace()?),
        Command::FishCompletionSetup => commands::completions::fish::print(),
        Command::FishCompletion => commands::completions::fish::tasks(load_workspace()?),
    })
}

fn load_workspace() -> Result<Workspace, Outcome> {
    let stacks = stacks::identify();
    if stacks.is_empty() {
        return Err(Outcome::UnknownStack);
    };
    Ok(Workspace { stacks })
}

fn flatten(outcome: Result<Outcome, Outcome>) -> Outcome {
    match outcome {
        Err(outcome) | Ok(outcome) => outcome,
    }
}
