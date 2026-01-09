# Raspberry Pi Pico WH × embedded rust

#### 1. 環境構築

`すでにRustはインストールしているものとします。`

- 1-1.　 Rust で Raspberry Pi Pico 用バイナリをビルドするためのターゲット追加

```ps1
rustup target add thumbv6m-none-eabi
```

- 1-2.　 UF2 形式への変換ツール elf2uf2-rs をインストール（--locked で Cargo.lock に従う）

```ps1
cargo install elf2uf2-rs --locked
```

#### 2. pico uf2 作成コマンド

- led blink sample

```ps1
cargo run --release  --bin led
```

- hello world sample

```ps1
cargo run --release  --bin helloworld
```
