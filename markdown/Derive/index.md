# Derive（継承）

ある型にふるまいを追加することができる。

## Debug

```rs
struct Data {
    value: i32
}

fn main() {
    let d = Data { value: 2 };
    println!("{:?}", d);
}

/*
error[E0277]: `Data` doesn't implement `Debug`
  --> src/main.rs:10:22
   |
10 |     println!("{:?}", d);
   |                      ^ `Data` cannot be formatted using `{:?}`
   |
   = help: the trait `Debug` is not implemented for `Data`
   = note: add `#[derive(Debug)]` to `Data` or manually `impl Debug for Data`
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `Data` with `#[derive(Debug)]`
   |
4  | #[derive(Debug)]
   |

For more information about this error, try `rustc --explain E0277`.
error: could not compile `playground` due to previous error
*/
```

`#[derive(Debug)]`を追加すると`{:?}`が使える。

```rs
#[derive(Debug)]
struct Data {
    value: i32
}

fn main() {
    let d = Data { value: 2 };
    println!("{:?}", d);
}
//=> Data { value: 2 }
```

## PartialEq

オブジェクト同士が等価であるか。

https://qiita.com/Kogia_sima/items/6899c5196813cf231054


