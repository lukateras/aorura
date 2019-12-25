let
  nixpkgs = import ./nixpkgs.nix;
in

with nixpkgs {};

mkRelease (gitignoreSource ./.) {
  aarch64-linux-gnu-native = nixpkgs { system = "aarch64-linux"; };
  aarch64-linux-musl-cross = pkgsCross.aarch64-multiplatform-musl.pkgsStatic;
  x86_64-linux-gnu-native = nixpkgs { system = "x86_64-linux"; };
  x86_64-linux-musl-cross = pkgsCross.musl64.pkgsStatic;
}
