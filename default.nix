{pkgs}:
let
  cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
  package = cargoToml.package;
in
{
  bmssp = pkgs.rustPlatform.buildRustPackage rec {
    pname = package.name;
    version = package.version;

    src = ./.;

    cargoLock = {
      lockFile = ./Cargo.lock;
    };

    nativeBuildInputs = [
      pkgs.pkg-config
    ];

    buildInputs = [];

    meta = with pkgs.lib; {
      description = package.description;
      homepage = package.repository;
      license =
        if package.license == "MIT" then licenses.mit
        else if package.license == "Apache-2.0" then licenses.asl20
        else if package.license == "GPL-3.0" then licenses.gpl3
        else licenses.unfree;
      maintainers = package.authors;
    };
  };
}
