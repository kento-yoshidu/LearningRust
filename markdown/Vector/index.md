# 配列

配列はコンパイル時に大きさが決まっている必要があります。実行時に配列のサイズを変更したいなら、Vector型を利用する。

# Vector型

実行時に配列のサイズが分からない場合、Vector型を利用しましょう。

`vec!`マクロを使ってVecを定義します。`mut`をつけておけば変更が可能になります。

```rs
fn main() {
    let mut vec = vec![1, 2, 3, 4];
}
```

```rs
fn main() {
    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];

    println!("v1 address {:p}", &v1);
    //=> 0xd3586ffa60

    println!("v2 address {:p}", &v2);
    //=> 0xd3586ffa78
}
```

`0x78 - 0x60`は10進数で24なので、スタック上に24バイトデータがあることがわかります。Vector型のメモリー構造は`String`型と同じです。`len`と`capacity`には**要素数**が格納されます。

```rs
fn main() {
    let mut v1 = vec![1, 2, 3, 4];

    println!("v1 {:?}", v1.as_ptr());
    //=> 0x1f29e90d620

    println!("v1 {}", v1.len());
    //=> 4
    println!("v1 {}", v1.capacity());
    //=> 4
}
```

`insert`で要素を追加できます。

```rs
fn main() {
    let mut v1 = vec![1, 2, 3, 4];

    v1.insert(1, 100);
    v1.remove(0);

    println!("{:?}", v1);
    //=> [100, 2, 3, 4]
}
```

`append`でデータを連結する。渡す方のVectorを可変参照で渡します。

```rs
fn main() {
    let mut v1 = vec![1, 2, 3, 4];
    let mut v3 = vec![9, 10];

    v1.append(&mut v3);

    println!("{:?}, {:?}", v1, v3);
    //=> [1, 2, 3, 4, 9, 10], []
}
```
