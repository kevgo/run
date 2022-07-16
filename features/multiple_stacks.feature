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
        @echo Makefile task running
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

  @this
  Scenario: run a task that exists in two stacks
    When executing "a format"
    Then it prints:
      """
      The task "format" exists in several stacks: Makefile and Node.js (npm).
      Please prefix the stack name to specify the task.

      mf for "Makefile format"
      nf for "Node.js (npm) format"
      """
    And the exit code is 1
