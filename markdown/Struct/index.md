# 構造体

```rs
#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    live: bool
}

fn main() {
    let user1 = User {
        name: String::from("kento"),
        age: 35,
        live: true
    };

    println!("{:?}", user1);
    //=> User { name: "kento", age: 35, live: true }
}
```

`..`を使えば、似たようなオブジェクトを作成できる。この時、所有権が移動する。

```rs
`#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    live: bool
}

fn main() {
    let user1 = User {
        name: String::from("kento"),
        age: 35,
        live: true
    };

    let user2 = &User {
        ..user1
    };

    // println!("{:?}", user1);
    // user1.nameの所有権が移動しているのでuser1はダンプできない
    //=> error[E0382]: borrow of partially moved value: `user1`

    println!("{:?}", user2);
    //=> User { name: "kento", age: 35, live: true }
}
```

## タプル構造体

```rs
#[derive(Debug)]
struct User(u8, &'static str);

fn main() {
    let user1 = (10, "kento");

    println!("{:?}", user1);
    //=> (10, "kento")
}
```

## メソッド

```rs
struct User {
    name: &'static str,
    age: u8
}

impl User {
    fn get(&self) {
        println!("名前は{}、年齢は{}歳。", self.name, self.age);
    }
}

fn main() {
    let user1 = User {
        name: "kento",
        age: 35
    };

    user1.get();
    //=> 名前はkento、年齢は35歳。
}
```


## 関連関数

```rs
#[derive(Debug)]
struct Num {
    number: u8
}

impl Num {
    fn new(number: u8) -> Self {
        Self { number: number * 10 }
    }
}

fn main() {
    let n = Num::new(10);

    println!("{:?}", n);
    //=> Num { number: 100 }
}
```

## 列挙型

いずれかひとつをとる。

```rs
#[derive(Debug)]
enum Address {
    Tokyo,
    Osaka,
    Nagoya
}

fn main() {
    let home = Address::Tokyo;

    println!("{:?}", home);
    //=> Tokyo
}
```


```rs
#[derive(Debug)]
enum Address {
    Tokyo(String, u8),
    Osaka(String, u8)
}

fn main() {
    let home = Address::Tokyo(String::from("test"), 10);

    println!("{:?}", home);
    //=> Tokyo("test", 10)
}
```









