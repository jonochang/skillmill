{ lib
, rustPlatform
, pkg-config
, cmake
, openssl
, libgit2
, zlib
}:
rustPlatform.buildRustPackage {
  pname = "skillmill";
  version = "0.1.3";

  src = ./.;
  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = [
    pkg-config
    cmake
  ];

  buildInputs = [
    openssl
    libgit2
    zlib
  ];

  env = {
    OPENSSL_NO_VENDOR = "1";
    LIBGIT2_NO_VENDOR = "1";
  };

  cargoBuildFlags = [ "-p" "skillmill" ];

  meta = with lib; {
    description = "Constraint-driven worksheet generator";
    homepage = "https://github.com/jonochang/skillmill";
    license = licenses.mit;
    mainProgram = "skillmill";
  };
}
