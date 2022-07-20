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


## シャドーイング

同じ変数名で再度`let`を使うことができる。上書きしているがアドレスを確認すると違うアドレスを指しているので、それぞれがスタックに積まれていることが分かる。同じスコープ内でのみ有効。

```rust
pub fn vars() {
  let y = 5;
  println!("{:p}", &y);

  let y = y + 1;
  println!("{:p}", &y);

  let y = y * 6;
  println!("{:p}", &y);
  println!("{}", y);
}
/*
0x5172affa9c
0x5172affaf4
0x5172affb4c
36
*/
```

## タプル型

分割代入のように取り出せる。

```rust
pub fn vars() {
  let t1 = (500, 6.4, "Hello");

  let (x, y, z) = t1;
}
```

インデックスでも取り出せる。

```rust
pub fn vars() {
  let t1 = (500, 6.4, "Hello");

  println!("{}, {}, {}", t1.0, t1.1, t1.2);
}
//=> 500, 6.4, Hello
```


多次元タプル。`ref`を使ってポインターを取得する。

```rust
pub fn vars() {
  let mut t2 = ((0, 1), (2, 3));

  let (ref mut x1_ptr, _) = t2;

  println!("{:p}", x1_ptr);
  //=> 0xa6be5af6a0
}
```

値を書き換える時は`*`で参照外しを行う。

タプルを出力するときは`{:?}`とする。

```rust
pub fn vars() {
  let mut t2 = ((0, 1), (2, 3));

  let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;

  // 参照外し
  *x1_ptr = 5;
  *y1_ptr = -5;

  println!("{:?}", t2);
  //=> ((5, -5), (2, 3))
}
```

## 配列

各要素はスタックに積まれていく。

## 文字列スライス

`UTF-8`を採用（1バイト～4バイト）。

文字列スライス型に推論される。

```rust
pub fn vars() {
  let s1 = "helloこんにちは挨拶";
  let s2 = "hello";

  // スタック内の番地
  println!("{:p}", s1);
  println!("{:p}", s2);
  //0x7ff609c7d410
  //0x7ff609c7d42a
}
```



