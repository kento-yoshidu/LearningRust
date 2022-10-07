
Rustは、ガベージコレクターもなければ、典型的なランタイムも存在しません。

C、C++と違い、メモリー安全性が保証されています。

https://qiita.com/akito_tameto/items/ea7f10dbdc16b565b9b8


|特徴|内容|
|---|---|
|タイプ セーフ|コンパイラによって、型が間違っている変数に操作が適用されないことが保証されます。|
|メモリーセーフ|Rust ポインター ("参照" と呼ばれます) によって、常に有効なメモリーが参照されます。|
|スレッドセーフ|ボローチェッカーが参照を検証し、スレッドセーフが保証される|
|ゼロコスト抽象化|Rust では、イテレーションやインターフェイス、関数型プログラミングなどの高度な概念を、パフォーマンス コストをほとんど、またはまったくかけずに利用できます。 抽象化は、基になるコードを手動で記述した場合と同様に実行されます。|

https://qiita.com/msakuta/items/e8935cbb4d4fe90a47d1

https://zenn.dev/khale/articles/rust-beginners-catchup

https://atmarkit.itmedia.co.jp/ait/articles/2111/25/news008.html

https://qiita.com/kichion/items/d5d87b30176e1d2d5f54

https://qiita.com/ksato9700/items/312be99d8264b553b193

https://numb86-tech.hatenablog.com/entry/2021/05/22/195352

## Rustの関数

必ず1つの`main`関数が必要です。

```rust
fn main() {
    println!("Hello Rust");
}
//=> Hello Rust
```

関数宣言は`fn`キーワードを使用します。

標準出力は`println!`マクロを使用します。

## Rustの変数

変数は`let`キーワードを使用して宣言します。変数だけを宣言することもできますし、同時に値に変数を束縛することもできます。

```rust
fn main() {
    let i: i32;
    i = 100;

    let j = 1;
}
```

# Rustの型

型を判定する関数。

```rust
fn type_name<T>(_: T) -> String {
    let t = std::any::type_name::<T>();
    return t.to_string();
}
```

使い方は以下の通り。

```rust
fn main() {
    let t = String::from("hoge");
    println!("{}", type_name(t));
    //=> alloc::string::String

    let t2 = "hoge";
    println!("{}", type_name(t2));
    //=> &str

    let i = 10;
    println!("{}", type_name(i));
    //=> i32

    let f = 1.1;
    println!("{}", type_name(f));
    //=> f64

    let b = true;
    println!("{}", type_name(b));
    //=> bool

    let arr = [1, 2, 3, 4, 5];
    println!("{}", type_name(arr));
    //=> [i32; 5]

    let vec = vec![6, 7, 8, 9, 10];
    println!("{}", type_name(vec));
    //=> alloc::vec::Vec<i32>

    let tup = (10, "10", false);
    println!("{}", type_name(tup));
    //=> (i32, &str, bool)
}
```

https://www.mochinoki-labo.com/rust-array-vec-intro/

