Feature: support multiple stacks

  Background:
    Given a file "package.json" with content:
      """
      {
        "name": "test",
        "scripts": {
          "format": "echo Node task running"
        }
      }
      """
    And a file "package-lock.json"
    And a Makefile with content:
      """
      format:  # formats the code
        echo Makefile task running
      """

  Scenario: list available tasks
    When executing "a"
    Then it prints:
      """
      Makefile

        format  formats the code

      Node.JS (npm)

        format  echo Node task running
      """

  Scenario: run a task
    When executing "a format"
    Then it prints:
      """
      formatting
      """
    Then the exit code is 0
