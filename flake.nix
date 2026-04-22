{
  description = "Walkers game devenv";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        toolchain = fenix.packages.${system}.combine [
          fenix.packages.${system}.stable.withComponents [
            "cargo"
            "rustc"
            "rust-src"
            "clippy"
          ]
          fenix.packages.${system}.latest.rustfmt
        ];
      in
        {
          devShells.default = pkgs.mkShell {
            nativeBuildInputs = [ toolchain ];

            shellHook = ''
              echo "Прошу вас, сделайте мне красиво!"
              cargo --version
            '';
          };
        }
    );
}