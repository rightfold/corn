{ nixpkgs ? import ./nix/nixpkgs.nix {} }:
nixpkgs.stdenv.mkDerivation {
    name = "corn-shell";
    buildInputs = [
        nixpkgs.rustc
        nixpkgs.cargo
    ];
}
