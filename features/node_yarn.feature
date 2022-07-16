Feature: Node.JS with Yarn

  Background:
    Given a file "package.json" with content:
      """
      {
        "name": "demo",
        "scripts": {
          "task-1": "echo one",
          "task-2": "echo two",
          "failing": "echo 'running a failing task' && exit 2"
        }
      }
      """
    And a file "yarn.lock"

  Scenario: list available tasks
    When executing "a"
    Then it prints:
      """
      Node.JS (yarn)

        failing  echo 'running a failing task' && exit 2
        task-1   echo one
        task-2   echo two
      """

  Scenario: run a task
    When executing "a task-1"
    Then it prints:
      """
      one
      """
    Then the exit code is 0

  Scenario: run an unknown task
    When executing "a zonk"
    Then it prints:
      """
      Error: task "zonk" doesn't exist

      Node.JS (yarn)

        failing  echo 'running a failing task' && exit 2
        task-1   echo one
        task-2   echo two
      """
    Then the exit code is 1

  Scenario: a task returns a non-zero exit code
    When executing "a failing"
    Then it prints:
      """
      running a failing task
      """
    And the exit code is 2

  Scenario Outline: setup
    When executing "a <COMMAND>"
    Then the output contains "yarn install"
    And the exit code is 0
    And the workspace contains a folder "node_modules"

    Examples:
      | COMMAND |
      | -s      |
      | --setup |
