{
  lib,
  rustPlatform,
  gtk4,
  pkg-config,
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "gtk-print-rs";
  version = "0.1.0";

  src = lib.cleanSource ./.;
  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
    gtk4
  ];

  meta = {
    description = "Print in the CLI using GTK";
    homepage = "https://github.com/RisGar/gtk-print-rs";
    license = lib.licenses.eupl12;
    platforms = lib.platforms.unix;
  };
})
