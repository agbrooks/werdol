{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11";
    naersk.url = "github:nix-community/naersk";
    nixpkgs-mozilla = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    } ;
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, naersk, flake-utils, nixpkgs-mozilla }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = (import nixpkgs) {
        inherit system;
        overlays = [(import nixpkgs-mozilla)];
      };
      toolchain = (pkgs.rustChannelOf {
        date = "2023-02-20";
        channel = "nightly";
        sha256 = "sha256-x7w/OUVweXcofRLYKpDGubBngiOqQG5ZZPRfzpnTGf8=";
      }).rust;
      naersk' = pkgs.callPackage naersk {
        cargo = toolchain;
        rustc = toolchain;
      };
      LIBCLANG_PATH = "${pkgs.llvmPackages_14.libclang}/lib";
    in {
      packages.default = naersk'.buildPackage {
        name = "werdol";
        src = ./.;
        nativeBuildInputs = with pkgs; [
          cmake
          pkg-config
          vulkan-tools
        ];
        buildInputs = with pkgs; [
          alsa-lib
          fontconfig
          udev
          wayland
          wrapGAppsHook
          xlibsWrapper
          libxkbcommon

          vulkan-loader
        ];
        inherit LIBCLANG_PATH;

        LD_LIBRARY_PATH="${pkgs.vulkan-loader}/lib";

        # definitely *shouldn't* go in bin, but eh, whatever
        postInstall = ''
        cp -R assets $out/bin/
        wrapProgram $out/bin/werdol --prefix 'LD_LIBRARY_PATH' ':' '${pkgs.vulkan-loader}/lib'
        '';

        # NB: will need some tweaks for MacOS
        meta.broken = pkgs.stdenv.isDarwin;
        passthru.exePath = "/bin/werdol";
      };

      devShell = with pkgs; mkShell {
        inputsFrom = [self.packages."${system}".default];
        nativeBuildInputs = [
          clippy
          rust-analyzer
          rustfmt
        ];
        RUST_SRC_PATH = rustPlatform.rustLibSrc;
        RUST_BACKTRACE = "1";

        # HACK: wgpu winds up picking a strange assortment of paths to search
        # when attempting to find a GPU driver. Give it a lil push.
        LD_LIBRARY_PATH="${pkgs.vulkan-loader}/lib";
        inherit LIBCLANG_PATH;
      };

      # Enables 'nix run'.
      apps.default = flake-utils.lib.mkApp {
        drv = self.packages.${system}.default;
      };
    }
  );
}
