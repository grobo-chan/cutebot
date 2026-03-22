# Copyright (C) 2026 GroboChan
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published
# by the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.
{
  description = "A flake to run GroboChan's cute discord bot";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = {
    self,
    nixpkgs,
    naersk,
  }: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
    naerskLib = pkgs.callPackage naersk {};
  in {
    devShells."x86_64-linux".default = pkgs.mkShell {
      buildInputs = with pkgs; [
        cargo
        rustc
        clippy
        rustfmt
        rust-analyzer
        sqlx-cli
        sqlite
      ];

      nativeBuildInputs = [pkgs.pkg-config];

      env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    };

    packages."x86_64-linux".default = naerskLib.buildPackage {
      src = ./.;
      buildInputs = with pkgs; [
        sqlite
        openssl
        libclang
      ];
      nativeBuildInputs = with pkgs; [
        pkg-config
        nasm
        cmake
        rustPlatform.bindgenHook
      ];
      AWS_LC_SYS_NO_AR = "1";
      LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
      BINDGEN_EXTRA_CLANG_ARGS = "-I${pkgs.openssl.dev}/include";
    };
  };
}
