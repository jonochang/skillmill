Feature: SkillMill CLI

  Scenario: Help output
    Given the skillmill CLI is available
    When I run "skillmill --help"
    Then the command succeeds
    And the output includes "skillmill"
