{
  description = "skillmill - constraint-driven worksheet factory";

  inputs = {
    nixpkgs.url     = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay    = {
      url    = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays     = [ (import rust-overlay) ];
        pkgs         = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "clippy" "rustfmt" "rust-src" ];
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain

            # Rendering
            pkgs.typst

            # Cargo dev tools
            pkgs.cargo-nextest
            pkgs.cargo-insta
            pkgs.cargo-deny
            pkgs.cargo-llvm-cov

            # Utilities
            pkgs.git
            pkgs.jq
          ];

          shellHook = ''
            if git rev-parse --git-dir > /dev/null 2>&1; then
              git config core.hooksPath .githooks
            fi
            echo "SkillMill dev shell ready. Run 'cargo build' to get started."
          '';
        };
      }
    );
}
