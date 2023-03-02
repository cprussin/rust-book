{
  sources ? import ./sources.nix,
  nixpkgs ? sources.nixpkgs,
  niv ? sources.niv,
  mkCli ? sources.mkCli,
}: let
  niv-overlay = self: _: {
    niv = self.symlinkJoin {
      name = "niv";
      paths = [niv];
      buildInputs = [self.makeWrapper];
      postBuild = ''
        wrapProgram $out/bin/niv \
          --add-flags "--sources-file ${toString ./sources.json}"
      '';
    };
  };

  mkCli-overlay = import "${mkCli}/overlay.nix";

  pkgs = import nixpkgs {
    overlays = [
      niv-overlay
      mkCli-overlay
    ];
  };

  cargo-with = plugins:
    pkgs.symlinkJoin {
      name = "cargo-with-plugins";
      paths = [pkgs.cargo];
      buildInputs = [pkgs.makeWrapper];
      postBuild = ''
        wrapProgram $out/bin/cargo \
          --prefix PATH : ${pkgs.lib.makeBinPath ([pkgs.cargo] ++ plugins)}
      '';
    };

  cli = pkgs.lib.mkCli "cli" {
    _noAll = true;

    run = "${pkgs.cargo}/bin/cargo run";

    release = "${pkgs.cargo}/bin/cargo build --release";

    docs = "${pkgs.cargo}/bin/cargo doc --open";

    test = {
      rust = {
        audit = "${cargo-with [pkgs.cargo-audit]}/bin/cargo audit";
        check = "${pkgs.cargo}/bin/cargo check";
        format = "${cargo-with [pkgs.rustfmt]}/bin/cargo fmt --check";
        lint = "${cargo-with [pkgs.clippy]}/bin/cargo clippy";
        unit = "${pkgs.cargo}/bin/cargo test";
        version-check = "${cargo-with [pkgs.cargo-outdated]}/bin/cargo outdated";
      };
      nix = {
        dead-code = "${pkgs.deadnix}/bin/deadnix .";
        format = "${pkgs.alejandra}/bin/alejandra --check .";
        lint = "${pkgs.statix}/bin/statix check .";
      };
    };

    fix = {
      rust = {
        format = "${cargo-with [pkgs.rustfmt]}/bin/cargo fmt";
        lint = "${cargo-with [pkgs.clippy]}/bin/cargo clippy --fix --allow-staged";
      };
      nix = {
        dead-code = "${pkgs.deadnix}/bin/deadnix -e .";
        format = "${pkgs.alejandra}/bin/alejandra .";
        lint = "${pkgs.statix}/bin/statix fix .";
      };
    };
  };
in
  pkgs.mkShell {
    nativeBuildInputs = [pkgs.pkg-config];
    buildInputs = [
      cli
      pkgs.git
      pkgs.niv
    ];
  }
