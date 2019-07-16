with import <nixpkgs> {};

stdenv.mkDerivation {
    name = "boolfuck-interpreter";

    buildInputs = [
        pkgs.cargo
        pkgs.rustup
        pkgs.rustracer
        pkgs.rustfmt
    ];
}