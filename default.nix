{ postgresql, sass, rustPlatform, buildBowerComponents, writeTextDir, cmake, openssl }:

let
  bowerPkgs = buildBowerComponents {
    name = "jude.bio";
    src = writeTextDir "bower.json" (builtins.readFile ./bower.json);
    generated = ./generated/bower.nix;
  };

in rustPlatform.buildRustPackage {
  name = "jude.rs";
  depsSha256 = "0pz77c9f6726d462jfmdqw5ba8xkaf4p93shm0kg09cpl7r7s6fm";
  src = ./.;
  buildInputs = [ postgresql sass cmake openssl ];
  shellHook = ''
    ln -sfv ${bowerPkgs}/bower_components .
  '';
  buildPhase = ''
    mkdir $out
    cargo install --root $out
    mkdir -p $out/lib
    cp $(find target -name 'libonig.*' | head -1) $out/lib
  '';
  installPhase = ":";
}
