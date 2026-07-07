[![wakatime](https://wakatime.com/badge/user/452b0d4c-9951-457d-868a-2007cd651d66/project/f15a341a-665e-41b1-a85e-3bf7c6f2dab4.svg)](https://wakatime.com/badge/user/452b0d4c-9951-457d-868a-2007cd651d66/project/f15a341a-665e-41b1-a85e-3bf7c6f2dab4)

# print-cli-rs

CLI Tool to open system print UI (NS on Darwin and GTK on Linux) for the passed PDF file.
Made for the purpose of using it inside a terminal file manager like [yazi](https://github.com/sxyazi/yazi).

## Installation

Use without installing:

```bash
nix run github:RisGar/print-cli-rs
```

Use in flake:

```nix
print-cli-rs = {
    url = "github:RisGar/print-cli-rs";
    inputs.nixpkgs.follows = "nixpkgs";
};
```
