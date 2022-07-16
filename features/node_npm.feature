Feature: Node.JS with npm

  Background:
    Given a file "package.json" with content:
      """
      {
        "name": "demo",
        "scripts": {
          "format": "echo formatting",
          "format:check": "echo checking format",
          "failing": "echo 'running a failing task' && exit 2"
        }
      }
      """
    And a file "package-lock.json"

  Scenario: list available tasks
    When executing "a"
    Then it prints:
      """
      Node.JS (npm)

        failing       echo 'running a failing task' && exit 2
        format        echo formatting
        format:check  echo checking format
      """

  Scenario: run a task
    When executing "a format"
    Then it prints:
      """
      formatting
      """
    Then the exit code is 0

  Scenario: run an unknown task
    When executing "a zonk"
    Then it prints:
      """
      Error: task "zonk" doesn't exist

      Node.JS (npm)

        failing       echo 'running a failing task' && exit 2
        format        echo formatting
        format:check  echo checking format
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
    Then the output contains "up to date"
    And the exit code is 0

    Examples:
      | COMMAND |
      | -s      |
      | --setup |
