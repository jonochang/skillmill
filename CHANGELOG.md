# Changelog

All notable changes to this project will be documented in this file.

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

