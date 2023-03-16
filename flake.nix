{
  outputs = { self, nixpkgs }: {
    devShell.x86_64-linux =
      let
        pkgs = nixpkgs.outputs.legacyPackages.x86_64-linux;
      in pkgs.mkShell {
        # nativeBuildInputs is usually what you want -- tools you need to run
        nativeBuildInputs = with pkgs; [
          pkg-config
          xorg.xhost
        ];
        buildInputs = with pkgs; [
          gtk4
        ];
    };
  };
}

