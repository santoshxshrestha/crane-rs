{
  description = "A nix flake for a Rust project using Actix-web framework";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-unstable";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { nixpkgs, naersk, ... }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      naerskLib = pkgs.callPackage naersk { };
    in {
      packages.${system}.default = naerskLib.buildPackage {
        src = ./.;
        buildInputs = [ pkgs.openssl ];
        nativeBuildInputs = [ pkgs.pkg-config ];
      };
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustc
          cargo
          rust-analyzer
          clippy
          cargo-watch
          rustfmt

          openssl

          yarn
          nodejs
          pnpm
          # nodePackages.typescript
          nodePackages.eslint
          nodePackages.prettier
          nodePackages.typescript-language-server
        ];

        nativeBuildInputs = [ pkgs.pkg-config ];

        # env.RUST_SRC_PATH =
        #   "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
      formatter = pkgs.rustfmt;
    };
}
