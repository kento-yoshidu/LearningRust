# 変数

`const`で定数を宣言できる。型を明示すること。

```rust
fn main() {
    const MAX_COUNT: i32 = 100;

    println!("{}", MAX_COUNT);
    //=> 100
}

変数の値が格納されているメモリーアドレスを知るには、`{:p}`プレースポルダーを使用する。

```rust
fn main() {
    println!("{:p}", &MAX_COUNT);
    //=> 0x7ff6607ad468
```

前述したとおり、定数はメモリーのStatic領域にバイナリーとして格納される。


以下のような変数はStackに格納される。

```rust
fn main() {
    let i2: i64 = 1;
    let i3: i64 = 2;
}
```

スタックのアドレスは以下のようにして取得できる。出力された値は、スタックの先頭アドレス。`i64`は8バイトなので、8バイト分スタックのアドレスがずれていることもわかる。

```rust
fn main() {
    let i2: i64 = 1;
    let i3: i64 = 2;

    println!("i2 is {:p}", &i2);
    println!("i3 is {:p}", &i3);
    //=> i2 is 0x471b13f568
    //=> i3 is 0x471b13f570
```

# シャドーイング

同じスコープで変数を静定義した場合、最後に記述したものが有効になり、それ以前のものは隠れてしまう。

以下の例の通り、再代入するとスタックアドレスも変わる。

```rust
fn main() {
    let y = 5;
    println!("{:p}", &y);
    //=> 0xdd3c4ff6ac

    let y = y + 5;
    println!("{:p}", &y);
    //=> 0xdd3c4ff704
}
```

# タプル

`()`を使って定義する。分割代入のように取り出す方法と、

```rust
fn main() {
    let t1 = (500, 6.4, "dummy");
    let (x, y, z) = t1;

    println!("{}, {}, {}", x, y, z);
    //=> 500, 6.4, dummy
}
```

インデックスを使って取り出す方法がある。

```rust
fn main() {
    let t1 = (500, 6.4, "dummy");
    let (x, y, z) = t1;

    println!("{}, {}, {}", t1.0, t1.1, t1.2);
    //=> 500, 6.4, dummy
}
```

タプルや構造体は`{:?}`を使って出力できる。

```rust
fn main() {
    let t1 = (500, 6.4, "dummy");

    println!("{:?}", t1);
    //=> (500, 6.4, "dummy")
```

多次元のタプルを用意。`mut`を付ければ変更可能。

```rust
fn main() {
    let mut t2 = ((0, 1), (2, 3));
}
```

`ref`キーワードをつけることで、分割代入時にアドレスを受け取ることもできる。

```rust
fn main() {
    let t2 = ((0, 1), (2, 3));

    let ((ref x1_ptr, ref y1_ptr), _) = t2;

    println!("{:p}, {:p}", &x1_ptr, &y1_ptr);
    //=> 0xf1aaddf528, 0xf1aaddf530
}
```

## 参照外し

普通に代入、スタックアドレスは変わらない。

```rust
fn main() {
    let mut t2 = (0, 1);

    println!("{:p}, {:p}", &t2.0, &t2.1);
    //=> 0xaeb7bff440, 0xaeb7bff444

    t2.0 = 100;

    println!("{:p}, {:p}", &t2.0, &t2.1);
    //=> 0xaeb7bff440, 0xaeb7bff444
}
```

参照から実データにアクセスするには、参照の先頭に`*`を付ける。

```rust
fn main() {
    let mut t2 = (0, 1);

    let (ref mut x1_ptr, ref mut y1_ptr) = t2;

    println!("{:p}, {:p}", &x1_ptr, &y1_ptr);
    //=> 0x2cb3d3f628, 0x2cb3d3f630

    *x1_ptr = 100;
    *y1_ptr = 200;

    println!("{:p}, {:p}", &x1_ptr, &y1_ptr);
    //=> 0x2cb3d3f628, 0x2cb3d3f630
}
```

参照外しの意味がイマイチ分からない。

# 配列

`let arr = [1, 2, 3, 4,5 ]`のように配列を定義できる。この時、型は`[i32; 5]`という風に推論される。`[0; 10]`とすれば、`0`が10個並んだ配列を作れる。

```rust
fn main() {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [0; 10];
}
```

タプルと同じように`{:?}`で配列をダンプできる。

```rust
fn main() {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [0; 10];

    println!("{:?}, {:?}", arr1, arr2);
    //=> [1, 2, 3, 4, 5], [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
}
```

`[]`でインデックスを使ってアクセスもできる。


