{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils}:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in with pkgs; {
        devShells.default = mkShell {
          buildInputs = [
            elixir
            elixir-ls
            inotify-tools
          ];
          DATABASE_URL = "postgresql://postgres:postgres@127.0.0.1:5908/splitter";
        };
      });
}
