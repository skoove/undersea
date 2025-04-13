{
  description = "Development shell for undersea";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix }:
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    fe_pkgs = fenix.packages.${system};
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        (fe_pkgs.stable.withComponents [
          "cargo"
          "clippy"
          "rust-src"
          "rustc"
          "rustfmt"
          "rust-analyzer"
        ])
        clang

        openssl # needed by reqwest
        pkg-config
      ];
    };
  };
}
