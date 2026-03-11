{
description = "Rust project with webkitgtk-4.2";

inputs = {
nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
flake-utils.url = "github:numtide/flake-utils";
};

outputs = { self, nixpkgs, flake-utils }:
flake-utils.lib.eachDefaultSystem (system:
let
pkgs = import nixpkgs { inherit system; };
in {
devShells.default = pkgs.mkShell {
buildInputs = with pkgs; [
rustc
cargo
pkg-config

        webkitgtk_4_2
        gtk3
        glib
        libsoup_3
      ];

      shellHook = ''
        export WEBKIT_DISABLE_COMPOSITING_MODE=1
      '';
    };
  });

}
