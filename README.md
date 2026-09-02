# ASTTE

## Versioned builds

The simplest way to build a version without editing the source is:

```powershell
.\build.ps1 1.2 --release
```

The script assigns the version to `ASTTE_VERSION` and forwards the remaining
arguments to `dx build`. The update checker accepts both `1.2` and `1.2.0`, as
well as GitHub tags such as `V1.2`.

## Arch Linux build

Run this on Arch Linux or an Arch-based CI runner:

```bash
sudo pacman -S --needed base-devel rustup pkgconf webkit2gtk-4.1
rustup default stable
cargo install dioxus-cli --locked
ASTTE_VERSION=1.2 dx build --release
```

The Linux build uses WebKitGTK, so `webkit2gtk-4.1` is required. Building on
Windows does not produce a native Linux desktop binary; use an Arch machine,
virtual machine, container, or CI runner.
