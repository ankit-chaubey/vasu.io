<div align="center">

```
  ██╗   ██╗ █████╗ ███████╗██╗   ██╗
  ██║   ██║██╔══██╗██╔════╝██║   ██║
  ██║   ██║███████║███████╗██║   ██║
  ╚██╗ ██╔╝██╔══██║╚════██║██║   ██║
   ╚████╔╝ ██║  ██║███████║╚██████╔╝
    ╚═══╝  ╚═╝  ╚═╝╚══════╝ ╚═════╝
```

**Ankit Chaubey's personal power toolkit** — written in Rust ⚡

[![Crates.io](https://img.shields.io/crates/v/vasu?color=cyan&style=flat-square)](https://crates.io/crates/vasu)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow?style=flat-square)](LICENSE)
[![CI](https://github.com/ankit-chaubey/vasu.io/actions/workflows/release.yml/badge.svg)](https://github.com/ankit-chaubey/vasu.io/actions)

</div>

---

## Install

### From crates.io
```bash
cargo install vasu
```

### Download binary (no Rust needed)

| Platform | Download |
|---|---|
| Linux x86-64 | `vasu-linux-x86_64` |
| Linux ARM64 | `vasu-linux-aarch64` |
| **Android / Termux** | `vasu-android-aarch64` |
| macOS Apple Silicon | `vasu-macos-arm64` |
| macOS Intel | `vasu-macos-x86_64` |
| Windows | `vasu-windows-x86_64.exe` |

### Termux (Android) quick install
```bash
curl -Lo vasu https://github.com/ankit-chaubey/vasu.io/releases/latest/download/vasu-android-aarch64
chmod +x vasu
mv vasu $PREFIX/bin/
```

---

## Commands

### `vasu`
Show the banner, version, and all available commands.

---

### `vasu del <keep…>`
Delete **everything** in CWD **except** the listed items.

```bash
vasu del .git vasu       # keeps .git/ and vasu/, nukes the rest
vasu del .git vasu -y    # skip confirmation
```

---

### `vasu cp <src> <dst>`
Deep-copy a folder — hidden files, symlinks, permissions included.

```bash
vasu cp project/  backup/
vasu cp /tmp/a    /tmp/b  --overwrite
```

---

### `vasu cb [targets…]`
Copy file contents to clipboard. Falls back to stdout in headless environments (Termux).

```bash
vasu cb *                         # everything in cwd recursively
vasu cb *.rs *.toml               # globs
vasu cb src/ README.md            # mix of dir + file
vasu cb doc video . xyz/op.html   # exact list
```

---

### `vasu tree [dir] [-d depth] [-a]`
Pretty colored directory tree.

```bash
vasu tree
vasu tree src/ -d 3
vasu tree -a               # show hidden files
```

---

### `vasu find <pattern> [dir] [-t f|d|all]`
Find files/dirs by name pattern (supports `*`).

```bash
vasu find "*.rs"
vasu find "config*" src/ -t f
```

---

### `vasu size [dir] [-n 20]`
Disk usage per item, sorted largest first.

```bash
vasu size
vasu size ~/Downloads -n 10
```

---

### `vasu clean [dir] [-y]`
Remove `__pycache__`, `.pyc`, `.DS_Store`, `target/`, `.pytest_cache`, build junk, etc.

```bash
vasu clean
vasu clean my_project/ -y
```

---

### `vasu zip <src> [out.zip]` / `vasu unzip <archive> [dest]`
Archive and extract.

```bash
vasu zip my_project/
vasu unzip release.zip extracted/
```

---

### `vasu rename <pattern> <replacement> [dir] [-n]`
Bulk rename files (string replace in filenames).

```bash
vasu rename " " "_"           # spaces → underscores
vasu rename ".jpeg" ".jpg"
vasu rename "test_" "" src/ --dry-run
```

---

### `vasu count [dir] [-e ext]`
Count files + lines of code by extension.

```bash
vasu count
vasu count src/ -e rs -e toml
```

---

### `vasu hash <file>`
Show MD5 + SHA256.

```bash
vasu hash release.zip
```

---

### `vasu backup [src] [--dest dir]`
Timestamped zip backup.

```bash
vasu backup
vasu backup my_project/ --dest ~/backups/
```

---

### `vasu env [filter]`
Print env vars, optionally filtered.

```bash
vasu env
vasu env PATH
```

---

### `vasu http [port] [dir]`
Zero-dependency HTTP file server with directory listing.

```bash
vasu http
vasu http 3000
vasu http 9000 dist/
```

---

### `vasu diff <a/> <b/>`
Compare two directories.

```bash
vasu diff v1/ v2/
```

---

### `vasu dupe [dir]`
Find duplicate files by MD5 hash.

```bash
vasu dupe ~/Downloads
```

---

## Releasing a new version

```bash
# 1. Bump version in Cargo.toml
# 2. Commit + tag
git add Cargo.toml Cargo.lock
git commit -m "chore: bump to v0.1.0"
git tag v0.1.0
git push && git push --tags
# ✅ GitHub Actions builds binaries for ALL platforms + publishes to crates.io
```

> **First time crates.io setup:**
> Get a token at https://crates.io/settings/tokens
> Add it as a GitHub secret named `CARGO_REGISTRY_TOKEN`

---

## Build from source

```bash
git clone https://github.com/ankit-chaubey/vasu.io
cd vasu.io
cargo build --release
# binary at: ./target/release/vasu
```

---

## License

MIT © [Ankit Chaubey](https://github.com/ankit-chaubey)
