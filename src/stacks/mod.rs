use crate::domain::Stacks;

mod makefile;
mod node_npm;
mod node_yarn;
mod rust_cargo;

/// determines the stacks in the current workspace
pub fn identify() -> Stacks {
    let mut result = vec![];
    makefile::scan(&mut result);
    node_npm::scan(&mut result);
    node_yarn::scan(&mut result);
    rust_cargo::scan(&mut result);
    result
}
