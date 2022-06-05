Feature: Node.JS with npm

  Background:
    Given a file "package.json" with content:
      """
      {
        "name": "demo",
        "scripts": {
          "task1": "echo one",
          "task2": "echo two"
          "failing": "echo 'running a failing task' && exit 2"
        }
      }
      """
    And a file "package-lock.json"

  @this
  Scenario: list available tasks
    When executing "atalanta"
    Then it prints:
      """
      Node.JS with npm:

      task1    echo one
      task2    echo two
      failing  echo failing && exit 2
      """

  Scenario: run a task
    When executing "atalanta one"
    Then it prints:
      """
      one
      """
    Then the exit code is 0

  Scenario: run an unknown task
    When executing "atalanta zonk"
    Then it prints:
      """
      Error: task "zonk" doesn't exist

      Node.JS with npm:

      task1    echo one
      task2    echo two
      failing  echo failing && exit 2
      """
    Then the exit code is 1

  Scenario: a task returns a non-zero exit code
    When executing "atalanta failing"
    Then it prints:
      """
      running a failing task
      """
    And the exit code is 2
