[![wakatime](https://wakatime.com/badge/user/452b0d4c-9951-457d-868a-2007cd651d66/project/f15a341a-665e-41b1-a85e-3bf7c6f2dab4.svg)](https://wakatime.com/badge/user/452b0d4c-9951-457d-868a-2007cd651d66/project/f15a341a-665e-41b1-a85e-3bf7c6f2dab4)

# gtk-print-rs

CLI Tool to open native system print UI for the passed PDF file.
Made for the purpose of using it inside a terminal file manager like [yazi](https://github.com/sxyazi/yazi).

## Installation

Use without installing:

```bash
nix run github:RisGar/gtk-print-rs
```

Use in flake:

```nix
gtk-print-rs = {
    url = "github:RisGar/gtk-print-rs";
    inputs.nixpkgs.follows = "nixpkgs";
};
```
