use super::Task;
use std::fmt::Display;
use std::process::Command;

/// a technology stack that Atalanta knows about
pub trait Stack: Display {
    /// provides the command to set up this stack (binary, argv)
    fn setup(&self) -> Option<Command>;

    /// Provides all executable tasks for the codebase in the current directory.
    /// This only emits read references. The stack instance should own the task data.
    fn tasks(&self) -> &Vec<Task>;

    /// provides the task with the given name
    fn task_with_name(&self, name: &str) -> Option<&Task> {
        self.tasks().iter().find(|task| task.name == name)
    }

    /// provides the command to update this stack to the latest version (dependencies)
    fn update(&self) -> Option<Command>;
}

pub type Stacks = Vec<Box<dyn Stack>>;
