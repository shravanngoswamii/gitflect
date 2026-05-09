# gitflect

A fast Git-aware prompt for Bash and Zsh on Linux and macOS. Compact repository segment, no shell framework required.

```sh
curl -fsSL https://raw.githubusercontent.com/shravanngoswamii/gitflect/main/install.sh | sh
```

**[Documentation →](https://shravangoswami.com/gitflect/)**

## Quick reference

```sh
gitflect status          # print current Git segment
gitflect config          # show active configuration
gitflect config init     # create config file from defaults
gitflect config path     # print config file path
gitflect --version
```

Prompt segment example:

```text
user@host ~/project [main ≡ +1 ~0 -0 | !2 ~1 -0 ?]
```

## Development

```sh
cargo test
cargo clippy --all-targets -- -D warnings
cargo build --release && cp target/release/gitflect ~/.local/bin/gitflect
```
