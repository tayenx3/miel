<div align="center">
    <h1>Miel</h1>
    <img src="https://img.shields.io/badge/license-Apache--2.0-blue">
    <img src="https://img.shields.io/badge/status-0.0.1%20Pre--Alpha-purple" alt="status: 0.0.1 Pre-Alpha">
</div>

> **🚧 EARLY DEVELOPMENT 🚧:** Miel is in very early development. Syntax can change and documentation is nonexistent. Now is the perfect time to get involved to help Miel stabilize.

A cozy systems programming language.

```haskell
#Import "std/fs.mi"

Add :: callable(a: int, b: int): int {
    a + b
}

;; permission types help prove secure data access at compile-time
ReadFileToString :: callable(path: strbuf): strbuf @fs.FsRead {
    ;; read file
}

Main :: callable() @Root {
    x := 5                  ;; type inference
    y: int = 10             ;; explicit typing
    z := Add(x, Add(5, y))

    data: box [u8; 1024] = box([0; 1024])
    ;; affine types ensure memory safety
    SomeOperation(data)
    SomeOperation(data)     ;; error: `data` was already moved

    acquire fs.FsRead from Root
    contents = ReadFileToString("config.toml")
    release fs.FsRead       ;; release when not needed
    
    contents = ReadFileToString("config.toml")  ;; error: fs.FsRead is absent
}
```

## Why Miel?

No reason, really. Honestly, just use Rust, Odin or Jai. I made Miel as "the language I'd use."\
Miel wasn't designed to be blazingly fast or revolutionary. Just.. "for me."

You're probably better off with Rust, Odin or Jai.

## Installation

### Prerequisites

To install/build Miel, you will need:

- Rust (install via [rustup.rs](https://rustup.rs))
- Git (install via [git-scm.com](https://git-scm.com))

### Install for your user only (default)

```bash
cargo install --git https://github.com/tayenx3/miel.git

# Verification
miel --version
```

> **Note:** If miel is not found, ensure ~/.cargo/bin is in your PATH. Rustup usually adds this automatically.

### Install system-wide (for all users)

```bash
git clone https://github.com/tayenx3/miel.git
cd miel
cargo install --path .
sudo ln -sf ~/.cargo/bin/miel /usr/local/bin/miel

# Verification
miel --version
```

This symlinks the binary to /usr/local/bin, making it available to every user on the system.

### Building manually

If you prefer to build and place the binary yourself:

```bash
cargo build --release
# Binary should be located at ./target/release/miel
# You can copy it anywhere

# For user-only:
cp ./target/release/miel ~/.cargo/bin/

# For system-wide:
sudo cp ./target/release/miel /usr/local/bin/
```

### Uninstallation

```bash
# If installed via cargo install:
cargo uninstall miel

# If symlinked:
sudo rm /usr/local/bin/miel

# If manually copied, remove from wherever you placed it
```

## Contributing

Contributions, either small or large, are always valued.

More information at [CONTRIBUTING.md](./CONTRIBUTING.md).

## Philosophy

Miel was created just for the joy of creation (no, not a FNAF reference).

More on the philosphy at [PHILOSOPHY.md](./PHILOSOPHY.md).

## Roadmap

You can find the roadmap for Miel at [ROADMAP.md](./ROADMAP.md).

## License

Miel is licensed under [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0). See [LICENSE](./LICENSE) for more details.
