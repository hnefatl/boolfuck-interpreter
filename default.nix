let mozilla_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
    nixpkgs = import <nixpkgs> { overlays = [ mozilla_overlay ]; };
    rust1330 = nixpkgs.rustChannelOf { channel = "1.33.0"; };
    rustup-faker = import (nixpkgs.fetchFromGitHub {
        owner = "hnefatl";
        repo = "rustup-faker";
        rev = "1bb0a6f357727e43218399ea6624f12ada2c3047";
        sha256 = "0m8kp6hxd6b41p1ydyrc3p0i8xi4grqcns6jjpfjmpykay5d0n6m";
    });
in
    with nixpkgs;
    stdenv.mkDerivation {
        name = "boolfuck-interpreter";

        buildInputs = [
            # Required by VSCode's RLS extension
            rustup-faker
            rust1330.rust-analysis
            rust1330.rust-src
            rust1330.rust-std
            rust1330.rls-preview

            # v1.33.0 tools
            rust1330.cargo
            rust1330.rust
            rust1330.rustfmt-preview
            rust1330.clippy-preview
        ];

        # RLS fails to find libraries otherwise
        LD_LIBRARY_PATH = "${rust1330.rust}/lib:$LD_LIBRARY_PATH";
    }
