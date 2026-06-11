# Miel

![status: 0.0.1 Pre-Alpha](https://shields.io/badge/status-0.0.1%20Pre--Alpha-purple)

> **🚧 EARLY DEVELOPMENT 🚧:** Miel is in very early development. Syntax can change and documentation is nonexistent. Now is the perfect time to get involved to help Miel stabilize.

A cozy systems programming language.

```haskell
func Add :: callable(a: int, b: int): int {
    a + b
}

proc Main :: callable() {
    x := 5                  ;; type inference
    y: int = 10             ;; explicit typing
    z := Add(x, Add(5, y))

    data: box [u8; 1024] = box([0; 1024])
    ;; affine types ensure memory safety
    SomeOperation(data)
    SomeOperation(data)     ;; error: `data` was already moved
}
```

## Why Miel?

No reason, really. Honestly, just use Rust, Odin or Jai. I made Miel as "the language I'd use."\
Miel wasn't designed to be blazingly fast or revolutionary. Just.. "for me."

You're probably better off with Rust, Odin or Jai.

## Installation

Prerequisites:

- Rust (install through https://rustup.rs)
- Visual Studio Build Tools (for Rust, should be prompted for when installing Rust)
- Git (install through https://git-scm.com)

```bash
git clone https://github.com/tayenx3/miel.git
cd miel
cargo build --release
# result should be in ./target/release with the name miel.exe, probably
```

## License

Miel is licensed under [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0/). See [LICENSE](./LICENSE) for more details
