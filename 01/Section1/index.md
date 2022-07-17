# Rustの特徴

## メモリー安全性

C/C++なら`malloc`や`free`で手動でメモリー管理を行う必要がある。Rustは所有権の仕組みを用いて、メモリー安全性を言語使用として保証している。

## GC

ほとんどの言語ではGCを用いてメモリー管理をしている。C/C++、RustはGCを用いていない。

# プロジェクト作成

`carge`というパッケージマネージャーを使用する。

`cargo new rust-learning`

関数定義は`fn`。

`println`の末尾の`!`はマクロを意味しており、`println`マクロを呼び出している。

```rust
fn main() {
    println!("Hello, world!");
}
```

`cargo run`でビルドと実行を自動で行ってくれる。ビルドだけなら`cargo`バイナリは`target`に格納される。

## PackageとCrateとModule

Packageの中には複数のCrateを作ることができる。`bin crate`が実際に処理を実行する。`lib crate`はライブラリー。

# モジュール作成

エクスポートする側は`pub`を付けてパブリックにしておく。

```rust:title=vars.rs
pub fn run() {
  println!("Here is vars module");
}
```

インポートする側は`mod`でモジュールを読み込み、`::`で関数にアクセスする。

```rust:title=main.rs
mod vars;

fn main() {
    println!("Hello, world!");
    vars::run();
}
```




