# マクロ

> マクロは関数と違ってRustの第一級オブジェクトでない

> 簡潔に言うとRustのプログラムをプログラミングする仕組みです。 関数がデータを受け取ってデータを返すのに対してマクロはRustの構文の一部(構文木)を受け取ってRustの構文の一部を返します。

> マクロはRustコンパイラがコンパイル中に実行するので関数とは全然違うタイミングで動きます。 

https://keens.github.io/blog/2018/02/17/makurokurabu_rustshibu/

# プリミティブ型

- 整数
- 浮動小数点数
- 真偽値
- 文字

## 整数

|長さ|符号付き|符号なし|
|---|---|---|
|8ビット|i8|u8|
|16ビット|i16|u16|
|32ビット|i32|u32|
|64ビット|i64|u64|
|128ビット|i128|u128|

## 浮動小数点数

- f32
- f64

## 真偽値

- true
- false

## ヒープとスタック

まずはこの2つのメモリー領域の違い、そしてRustがどのようにこれらの領域を使い分けているかを確認します。

ヒープ領域はOSやアプリケーションから動的に確保、解放されるメモリーです。例えばC言語であれば`malloc()`でメモリーを割り当て`free()`で解放します。

対してスタックはLIFOでしかデータをやり取りできません。ただし、この分高速に動作します。

|領域|置かれるデータ|
|---|---|
|ヒープ|可変長|
|スタック|固定長|

https://uquest.tktk.co.jp/embedded/learning/lecture16.html

## スライス

Rustには**スライス**という概念がある。

スライスはコレクション内の一連の要素を**参照**したものです。コレクションはデータ型の一種で、Vec（可変長配列）やString（文字列）などです。

スライスはスタックに積まれます。


|名前|型|特徴|
|---|---|---|
|文字列リテラル|&str|コンパイル時にデータが決まっている|
|文字列型|String|データ長が分からないため、ヒープ領域に割り当てられる|

## タプル

```rust
fn main() {
    let t = ('E', 214, true);
    
    println!("{:?}", t);
    //=> ('E', 214, true)
    
    println!("{}", t.0);
    //=> 'E'
    
    println!("{}", t.1);
    //=> 214
    
    println!("{}", t.2);
    //=> true
}
```

## 構造体

従来の構造体

```rust
fn main() {
    struct Student { name: String, level: u8, remote: bool }
}
```

構造体のインスタンス化

```rust
fn main() {
    struct Student {
        name: String,
        level: u8,
        remote: bool
    }
    
    let user_1 = Student { name: String::from("kento"), level: 100, remote: true };
    
    println!("name is {}, level is {}, remote is {}.", user_1.name, user_1.level, user_1.remote);
    //=> name is kento, level is 100, remote is true.
}
```


## 所有権

所有権はリソースを解放する権利のことであり、開放しなければならない義務でもある。変数がスコープを抜けるときにリソースが解放される。

オブジェクトには**所有権**があり、２つの属性がある。

https://qiita.com/cactaceae/items/2c70a9947364c60ec100

https://keens.github.io/blog/2020/06/20/shadoingunoureshisa/

https://qiita.com/yoshii0110/items/0b8780416c30365db54c

https://qiita.com/nirasan/items/9e169859c6807c2c175b


> Rust では所有権を使ってオブジェクトを受け渡します．通常は所有権を渡してしまうと束縛が解除されて，受け取った側がそれを束縛します．そこで，仮の所有権を作成して相手に渡すことで，渡す側は束縛を解除されず，仮の所有権を受け取った側はその所有権を使ってオブジェクトを操作することが出来ます．そして，受け取った側の変数がスコープを外れた時に束縛していた仮の所有権が破棄されます．この時，原本または他の仮の所有権があればオブジェクトは破棄されません．仮の所有権を作成する方法の１つが 参照 （reference）です．これは & 演算子を使います．

## 束縛

`let`を使い、値に変数を**束縛**します。束縛された変数が所有者となり、所有権を有することになる。

## 参考

https://matsumaee.hatenablog.com/entry/2021/07/19/194550

## 所有権の移動

```rust
fn main() {
    let s1: String::from("hello");
    let s2 = s1;
}
```

ここで`s1`から`s2`への所有権の移動（move）が起こっている。

```rust
fn main() {
    let s1 = String::from("hello");
        // move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    let s2 = s1;
        // value moved here
    println!("{} {}", s1, s2);
                      ^^ value borrowed here after move
}
// For more information about this error, try `rustc --explain E0382`.
// error: could not compile `section2` due to previous error
```

moveが発生するのは**スタックのポインターのデータで、ヒープの実データのアドレスを指しているもの**。代表的なものは`String`型。上記エラーの通り、コピートレイトが実装されていないためエラーになる。

`String`型、`Vector`型、`Box`型。

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



