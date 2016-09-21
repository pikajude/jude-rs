{ jude-web ? { outPath = ./.; }

, supportedCompilers ? [ "rustStable" "rustBeta" "rustUnstable" ]
, supportedPlatforms ? [ "x86_64-linux" "i686-linux" ]
}:

{ build = let inherit ((import <nixpkgs> {}).lib) genAttrs; in

genAttrs supportedCompilers (compiler:
  genAttrs supportedPlatforms (system:
    with import <nixpkgs> { inherit system; };

    callPackage ./. {
      rustPlatform = makeRustPlatform pkgs."${compiler}";
    }
  )
); }
