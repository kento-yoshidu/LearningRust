# Hello World

## main.rsの作成

Hello Worldする。

```rs
// main.rs

fn main() {
    println!("Hello World");
}
```

1. main関数を作成する
2. println!という**マクロ**を呼び出している
3. 末尾には`;`が必要
4. スペース4つでインデントを作る

## コンパイル

```bash
# shell

$ rustc mainrs

$ ls

main.exe*  main.pdb  main.rs

$ ./main.rs
Hello World
```

# Rustシステム

## Cargoって？

Rustのビルドシステム兼パッケージマネージャー。

`cargo new <Project_name> --bin`でプロジェクトの作成ができる。

```shell
$ cargo new Hello_World --bin

$ find Hello_World

Hello_World/
Hello_World/Cargo.toml 
Hello_World/src        
Hello_World/src/main.rs
```

## toml

設定ファイルを書くための言語らしい。

[設定ファイル記述言語 TOML - Qiita](view-source:https://qiita.com/b4b4r07/items/77c327742fc2256d6cbe)

中身を見る限り、Nodeにおける`package.json`みたいな感じ？

```toml
[package]
name = "Hello_World"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

## cargoコマンド

チェック。スネークケースじゃないとダメと怒られた。次からはそうしよう。

```bash
$ cargo check
    Checking Hello_World v0.1.0 (C:\github\LearningRust\Hello_World)
warning: crate `Hello_World` should have a snake case name
  |
  = note: `#[warn(non_snake_case)]` on by default
  = help: convert the identifier to snake case: `hello_world`

warning: `Hello_World` (bin "Hello_World") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.47s
```

ビルド。

```bash
$ cargo build
   Compiling Hello_World v0.1.0 (C:\github\LearningRust\Hello_World)
warning: crate `Hello_World` should have a snake case name
  |
  = note: `#[warn(non_snake_case)]` on by default
  = help: convert the identifier to snake case: `hello_world`

warning: `Hello_World` (bin "Hello_World") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.47s
```

実行。

```bash
$ cargo run
warning: crate `Hello_World` should have a snake case name
  |
  = note: `#[warn(non_snake_case)]` on by default
  = help: convert the identifier to snake case: `hello_world`

warning: `Hello_World` (bin "Hello_World") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target\debug\Hello_World.exe`
Hello, world!
```