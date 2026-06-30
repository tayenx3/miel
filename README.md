<div align="center">
    <h1>Miel</h1>
    <img src="https://img.shields.io/badge/license-Apache--2.0-blue">
</div>

> **🚧 EARLY DEVELOPMENT 🚧:** Miel is in very early development. Syntax can change and documentation is nonexistent. Now is the perfect time to get involved to help Miel stabilize.

Miel is a cozy, resource-oriented programming language for creating provably safe and secure systems while maintaining ergonomics and practicality.

```haskell
cap Foo;

;; affine types allow for safe and efficient memory usage
#Affine
struct Bar @Foo {
  a: int,
  b: float,
  c: bool,
};
;; capability types help prove secure data access at compile-time
Baz :: callable(n: Bar) @Foo {
  ;; ...
};
;; lazy enforcement (checked at use-time)
Qux :: callable(n: Bar) { ;; no error!
  a := n; ;; NOW error: usage of type `Bar` requires capability `Foo`
  ;; ...
};

Main :: callable() @Root {
  acquire Foo from Root;
  bar := Bar {
    a: 0,
    b: 0.0,
    c: false,
  };
  
  Baz(bar);
  Baz(bar); ;; error: `bar` was moved

  bar2 := Bar {
    a: 69,
    b: 42.0,
    c: true,
  };
  
  release Foo; ;; release when unneeded (or let the compiler implicitly release at the end of scope)

  Baz(bar2); ;; error: usage of function `Baz` requires capability `Foo`
  Qux(bar2); ;; error: usage of type `Bar` requires capability `Foo`
};
```

## Why Miel?

No reason, really. Honestly, just use Rust, Odin or Jai. Maybe Austral too. I made Miel as "the language I'd use."\
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

Miel is made by humans, for humans *(yes, really.)* So contributions, either small or large, are always valued.

More information at [CONTRIBUTING.md](./CONTRIBUTING.md).

## Philosophy

Miel was created just for the joy of creation (no, not a FNAF reference).

More on the philosphy at [PHILOSOPHY.md](./PHILOSOPHY.md).

## Roadmap

You can find the roadmap for Miel at [ROADMAP.md](./ROADMAP.md).

## License

Miel is licensed under [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0). See [LICENSE](./LICENSE) for more details.
