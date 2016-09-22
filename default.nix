{ postgresql, sass, rustPlatform, buildBowerComponents, writeTextDir, cmake }:

let
  bowerPkgs = buildBowerComponents {
    name = "jude.bio";
    src = writeTextDir "bower.json" (builtins.readFile ./bower.json);
    generated = ./generated/bower.nix;
  };

in rustPlatform.buildRustPackage {
  name = "jude.rs";
  depsSha256 = "1xaad12370jzxmlhajivbg6siifrhyyzvqjqzb6ll9hahldw8zlc";
  src = ./.;
  buildInputs = [ postgresql sass cmake ];
  shellHook = ''
    ln -sfv ${bowerPkgs}/bower_components .
  '';
}
