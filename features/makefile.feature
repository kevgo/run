Feature: Makefiles

  Background:
    Given a Makefile with content:
      """
      format:  # formats the code
        echo "formatting"

      format-check:  # checks for formatting problems
        echo "check formatting"

      failing:  # this task returns a non-zero exit code
        echo "running a failing task"
        exit 2

      .SILENT:
      """

  Scenario: list available tasks
    When executing "a"
    Then it prints:
      """
      Makefile

        format        formats the code
        format-check  checks for formatting problems
        failing       this task returns a non-zero exit code
      """

  Scenario: run a task via full name
    When executing "a format"
    Then it prints:
      """
      formatting
      """
    Then the exit code is 0

  Scenario: run a task via shortcut
    When executing "a fc"
    Then it prints:
      """
      check formatting
      """
    Then the exit code is 0

  Scenario: multiple tasks match a shortcut
    When executing "a fo"
    Then it prints:
      """
      Multiple matches:
        format        formats the code
        format-check  checks for formatting problems
      """
    Then the exit code is 1

  Scenario: run an unknown task
    When executing "a zonk"
    Then it prints:
      """
      Error: task "zonk" doesn't exist

      Makefile

        format        formats the code
        format-check  checks for formatting problems
        failing       this task returns a non-zero exit code
      """
    Then the exit code is 1

  Scenario: a task returns a non-zero exit code
    When executing "a failing"
    Then it prints:
      """
      running a failing task
      """
    And the exit code is 2

  Scenario: setup
    When executing "a -s"
    Then it prints:
      """
      Warning: I don't know how to set up this workspace
      """
    And the exit code is 1

  Scenario: update
    When executing "a -u"
    Then it prints:
      """
      Warning: I don't know how to upgrade up this workspace
      """
    And the exit code is 1
