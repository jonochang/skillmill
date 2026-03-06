# Changelog

All notable changes to this project will be documented in this file.

## [0.1.3] - 2026-03-06
### Added
- Fraction curriculum expansion with componentized schemas for:
  - Language
  - Symbols
  - Diagrams
- Diagram-style fraction prompts using visual shaded bars in worksheet items.

### Changed
- Fraction generators now emit child-friendly prompts split by component and level progression.
- Worksheet and answer-key templates now group fraction content into explicit section columns
  (`Language`, `Symbols`, `Diagrams`) for clearer review and print flow.
- Crate versions and package version bumped to `0.1.3`.

## [0.1.2] - 2026-03-05
### Added
- Geometry curriculum coverage for P1-P3:
  - P1 2D shapes (number of sides)
  - P2 2D shapes (number of vertices/corners)
  - P3 3D solids (number of faces)
- Child-friendly geometry question wording and expanded prompt variants.
- Duplicate detector in worksheet composition with retry-and-drop behavior for repeated items.
- PNG export support in rendering and CLI output (including multi-page PNG naming).
- Root `package.nix` and `flake.nix` package output for release packaging.

### Changed
- Multiplication and division are now generated in vertical form with a top rule line.
- Typst worksheet and answer-key templates were refactored for improved pagination and layout stability.
- Crate versions bumped to `0.1.2`.

## [0.1.1] - 2026-03-05
### Added
- Cucumber BDD harness for the CLI with a first `--help` scenario.
- Crucible and Untangle added to the Nix flake dev shell (built from local inputs).
- P1–P3 worksheet batch generation scripts and policies used for review runs.

### Changed
- Refined Typst worksheet/answer key layout (landscape, dense 3-column, header/date styling, spacing fixes).
- Explicit schema registry for P1–P3 mapping to generators for clearer curriculum management.
- Difficulty biasing for add/sub generators to better differentiate P1 vs P2 vs P3.
- Auto-vertical formatting for 2+ digit add/sub problems.

## [0.1.0] - 2026-03-03
### Added
- Nix-based dev environment (`flake.nix`) with Rust toolchain and Typst.
- Rust workspace with `skillmill-core`, `skillmill` CLI, and `skillmill-math` plugin.
- Core data models, plugin trait, composer, and render pipeline.
- Singapore Math P1–P3 curriculum YAML and arithmetic schema generators.
- Typst templates for worksheets and answer keys (base + math discipline).
- CLI commands: `init profile`, `init policy`, `generate`, `preview`, `list`, `validate`.
- Validation and snapshot test harnesses, plus CI workflow.
- Bench harness for worksheet rendering performance.
- Git hooks and `cargo-deny` configuration.
