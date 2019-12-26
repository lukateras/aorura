{ pkgs ? import ./nixpkgs.nix {} }: with pkgs;

let
  inherit (rust.packages.nightly) rustPlatform;
in

{
  aorura-cli = buildRustPackage rustPlatform {
    name = "aorura-cli";
    src = gitignoreSource ./.;
    cargoDir = "cli";
  };

  aorura-emu = buildRustPackage rustPlatform {
    name = "aorura-emu";
    src = gitignoreSource ./.;
    cargoDir = "emu";
  };
}
