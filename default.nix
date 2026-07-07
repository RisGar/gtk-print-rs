{
  lib,
  rustPlatform,
  stdenv,
  pkg-config,
  gtk4,
  apple-sdk,
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "print-cli-rs";
  version = "0.1.0";

  src = lib.cleanSource ./.;
  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = lib.optionals stdenv.hostPlatform.isLinux [
    pkg-config
  ];

  buildInputs =
    lib.optionals stdenv.hostPlatform.isLinux [
      gtk4
    ]
    ++ lib.optionals stdenv.hostPlatform.isDarwin [
      apple-sdk
    ];

  meta = {
    description = "Print in the CLI on Darwin (NS) & Linux (GTK)";
    homepage = "https://github.com/RisGar/print-cli-rs";
    license = lib.licenses.eupl12;
    platforms = lib.platforms.linux ++ lib.platforms.darwin;
  };
})
