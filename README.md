# sikios

## What is this?

This is toy OS to study.

Rust 100% (At this point...)

## How to Build?

I'm working on build system, so it is not possible to build at this time.

This project is using `sagiegurari/cargo-make` to build.

### Before Run

If you are not installed `sagiegurari/cargo-make`, you can install.

```bash
cargo install --force cargo-make
```

### Run

```bash
export OVMF_PATH = "/usr/share/ovmf/OVMF.fd"

cargo make run
```
