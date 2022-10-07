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