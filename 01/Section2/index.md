# ニュータブルとイミュータブル

Rustは全ての変数がデフォルトでイミュータブルなので値を書き換えることができない。

```rust
pub fn vars() {
  let x = 5;
  println!("{}", x);

  x = 6;
  println!("{}", x);
}

// error[E0384]: cannot assign twice to immutable variable `x`
```

ミュータブルにするには、束縛時に`mut`を付ける。変数の値を出力するには`{}`を渡す。

```rust
pub fn vars() {
  let mut x = 5;
  println!("{}", x);
  //=> 5

  x = 6;
  println!("{}", x);
  //=> 6
}
```

宣言したもののその後使用していない変数のwarnnigを消すには、先頭に_を付ける。


## 型推論

整数なら`i32`（符号付32bit）、浮動小数点数なら`f64`。

## 定数

`const`を使用する。

```rust
const MAX_POINTS: u32 = 100_000;
```

定数はグローバルでも関数内でも定義できるが、変数は関数内のみで定義可能。

## メモリーのアドレス

アドレス演算子(`&`)と、`:p`を付ける。


```rust
const MAX_POINTS: u32 = 100_000;

pub fn vars() {
  println!("{:p}", &MAX_POINTS);
  //=> 0x7ff6ab1dd438
}
```

`i2`、`i3`はサイズが決まっているのでスタックに積まれていく。


