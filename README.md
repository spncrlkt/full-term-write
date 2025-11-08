# Rust development shortcuts
cargo run
cargo build
cargo test
cargo check
cargo watch -x run
cargo clippy -- -D warnings
cargo fmt

# Quick debugging
rust-gdb --args  # Install: cargo install rust-gdb


# .cargo/config.toml
```
[build]
target-dir = "target"  # Consistent across platforms

[alias]
dev = "run"
lint = "clippy -- -D warnings"
fmt = "fmt --all"

[target.'cfg(target_os = "macos")']
rustflags = ["-C", "link-arg=-Wl,-rpath,@executable_path/../lib"]
```
