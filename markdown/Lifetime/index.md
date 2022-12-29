# ライフタイム

参照を使っているときに登場する。Dangling Reference（既に解消されたメモリーを指している参照）。

```rs
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("{}", r);
    /*
      Compiling playground v0.0.1 (/playground)
      error[E0597]: `x` does not live long enough
        |
        |         r = &x;
        |             ^^ borrowed value does not live long enough
        |     }
        |     - `x` dropped here while still borrowed
        |
        |     println!("{}", r);
        |                    - borrow later used here
      For more information about this error, try `rustc --explain E0597`.
    */
}
```

`r`は`x`を参照しているが、その`x`は`{}`の中でドロップされているため、`println!()`で参照できない。

https://zenn.dev/ucwork/articles/6de5c9c2257f2d

## ライフタイムパラメーター

以下の場所に書くことができる。

```rs
&'a T
&'a mut T
T<'a>
```

以下のように明示的に指定する。

```rs
let a = 'a b;
```

```rs
fn longest(x: &String, y: &String) -> &String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x = String::from("hoge");
    let y = String::from("foobabar");

    println!("{}", longest(&x, &y));
}
```

> 戻り値の参照がx,yになるか、コンパイラが判別できないため、戻り値がどちらのライフタイムになるかわからないからです。

> この問題を解決するため、参照間の関係を定義するライフタイム引数を用います。

> 同じライフタイム引数では同じライフタイムを持つものとしてコンパイルされます。 longest関数の入力は同じライフタイムを持つものとして同じライフタイム引数を使用します。

```rs
fn longest<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x = String::from("hoge");
    let y = String::from("foobabar");

    println!("{}", longest(&x, &y));
}
```

https://qiita.com/deta-mamoru/items/e7d0b52f56b23b403c62
