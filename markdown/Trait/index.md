# トレイト

ある振る舞いを抽象化し、それを複数の型に持たせることができます。

`Number`構造体を定義します。

```rs
struct Number {
    number: i32
}
```

インスタンスの持つ`number`を倍にして返す`NumTrait`トレイトを定義します。`twice`というメソッドを定義しますが、実際にどんな処理を行うかはここでは定義しません。

```rs
trait NumTrait {
    fn twice(&self) -> i32;
}
```

続いて、`Number`に`NumTrait`トレイトを実装します。`impl`キーワードを使います。`(&self) -> i32`という型に適応している必要があります。

```rs
impl NumTrait for Number {
    fn twice(&self) -> i32 {
        &self.number * 2
    }
}
```


```rs
struct Number {
    number: i32
}

trait NumTrait {
    fn twice(&self) -> i32;
}

impl NumTrait for Number {
    fn twice(&self) -> i32 {
        &self.number * 2
    }
}

fn main() {
    let num = Number {
        number: 2
    };

    println!("{}", num.twice());
    //=> 4
}
```

## あるトレイトを実装していることを要求する

あるトレイトを持った型しか引数として受け入れない関数を作成することができます。


```rs
trait Summary {
    fn summarize(&self) -> String;
}

struct User {
    id: u8,
    name: String
}

impl Summary for User {
    fn summarize(&self) -> String {
        format!("id = {}.\nname={}.", self.id, self.name)
    }
}

fn print_user(user: &impl Summary) {
    println!("{}", user.summarize())
}

fn main() {
    let user1 = User {
        id: 1,
        name: String::from("Kento")
    };

    print_user(&user1);
}
```

`Summary`トレイトを実装しないただの`User`であれば、コンパイルエラーになります。

```rs
trait Summary {
    fn summarize(&self) -> String;
}

struct User {
    id: u8,
    name: String
}

/*
impl Summary for User {
    fn summarize(&self) -> String {
        format!("id = {}.\nname={}.", self.id, self.name)
    }
}
*/

fn print_user(user: &impl Summary) {
    println!("{}", user.summarize())
}

fn main() {
    let user1 = User {
        id: 1,
        name: String::from("Kento")
    };

    print_user(&user1);
    /*
      Compiling playground v0.0.1 (/playground)
      error[E0277]: the trait bound `User: Summary` is not satisfied
      |
      |     print_user(&user1);
      |     ---------- ^^^^^^ the trait `Summary` is not implemented for `User`
      |     |
      |     required by a bound introduced by this call
      |
      note: required by a bound in `print_user`
        --> src/main.rs:18:27
        |
      18 | fn print_user(user: &impl Summary) {
        |                           ^^^^^^^ required by this bound in `print_user`

      For more information about this error, try `rustc --explain E0277`.
    */
  }
```

しかも`the trait "Summary" is not implemented for "User"`という風に、型`User`に`Summary`トレイトが実装されてないよーとまで教えてくれています。すごい！

もしくはジェネリックを使ってもOK。

```rs
trait Summary {
    fn summarize(&self) -> String;
}

struct User {
    id: u8,
    name: String
}

impl Summary for User {
    fn summarize(&self) -> String {
        format!("id = {}.\nname={}.", self.id, self.name)
    }
}

fn print_user<T: Summary>(user: &T) {
    println!("{}", user.summarize())
}

fn main() {
    let user1 = User {
        id: 1,
        name: String::from("Kento")
    };

    print_user(&user1);
}
```


https://qiita.com/deta-mamoru/items/b9bc953b54d8eea605d5


## ジェネリックとトレイト境界

