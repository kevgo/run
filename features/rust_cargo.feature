Feature: Rust (Cargo)

  Background:
    Given a file "Cargo.toml" with content:
      """
      [package]
      name = "atalanta"
      version = "0.0.0"
      """
    And a file "Cargo.lock"

  Scenario: list available tasks
    When executing "a"
    Then it prints:
      """
      Rust (Cargo)

        build  cargo build
        check  cargo check
        test   cargo test
      """

  Scenario: run a task via full name
    When executing "a check"
    Then the exit code is 101

  Scenario: run a task via shortcut
    When executing "a c"
    Then the exit code is 101

  Scenario: multiple tasks match a shortcut
    When executing "a e"
    Then it prints:
      """
      Multiple matches:
        check  cargo check
        test   cargo test
      """
    Then the exit code is 1

  Scenario: run an unknown task
    When executing "a zonk"
    Then it prints:
      """
      Error: task "zonk" doesn't exist

      Rust (Cargo)

        build  cargo build
        check  cargo check
        test   cargo test
      """
    Then the exit code is 1

  @this
  Scenario: setup
    When executing "a -s"
    Then it prints:
      """
      Warning: I don't know how to set up this workspace
      """
    And the exit code is 1
