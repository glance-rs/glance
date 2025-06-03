{
  description = "Glance is a computer vision library written in pure Rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs: let
    supportedSystems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];
    forEachSupportedSystem = f:
      inputs.nixpkgs.lib.genAttrs supportedSystems (system:
        f {
          pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              inputs.rust-overlay.overlays.default
              inputs.self.overlays.default
            ];
          };
        });
  in {
    overlays.default = final: prev: {
      rustToolchain = let
        rust = prev.rust-bin;
      in
        if builtins.pathExists ./rust-toolchain.toml
        then rust.fromRustupToolchainFile ./rust-toolchain.toml
        else if builtins.pathExists ./rust-toolchain
        then rust.fromRustupToolchainFile ./rust-toolchain
        else
          rust.stable.latest.default.override {
            extensions = ["rust-src" "rustfmt" "clippy"];
          };
    };

    devShells = forEachSupportedSystem ({pkgs}: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          rustToolchain
          pkg-config
          rust-analyzer
          wayland
          wayland-protocols
          libxkbcommon
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          xorg.libXext
          xorg.libXrender
          xorg.libxcb
        ];

        env = {
          RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
        };
      };
    });
  };
}
