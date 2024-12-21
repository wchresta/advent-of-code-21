{ outputs = { nixpkgs, flake-utils, ... }:
  flake-utils.lib.eachDefaultSystem(system:
  let
    pkgs = import nixpkgs { inherit system; };
  in {
    devShells.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        cargo
        rustc
        rustfmt
        rust-analyzer

        # Needed if we want to use nalgebra-lapack
        # lapack
        # pkg-config
        # cmake
        # gfortran
        # blas

        # for new_day
        xmlstarlet
      ];

      # Certain Rust tools won't work without this
      # This can also be fixed by using oxalica/rust-overlay and specifying the rust-src extension
      # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela. for more details.
      RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    };
  });
}
