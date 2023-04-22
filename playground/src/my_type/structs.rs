// 出力するにはDebugトレイトを実装する
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// タプル構造体(フィールド名がない構造体)
#[derive(Debug)]
struct ColorRGB(u32, u32, u32);

// 構造体へのメソッドの実装
impl Person {
    // 関連関数（静的メソッド
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }

    // メソッド
    fn greeting(&self) {
        println!("I'm {}, {} years old", {&self.name}, {self.age});
    }
}

pub fn run() {
    // Person構造体のインスタンスを生成する
    let p1 = Person {
        name: String::from("kento"),
        age: 18
    };

    println!("1: Person構造体のインスタンス {:?}", p1);
    println!("1: インスタンスのnameフィールド {}", p1.name);
    println!("1: インスタンスのageフィールド {}", p1.age);

    // mutでフィールドの値を変更できる
    let mut p2 = Person {
        name: String::from("hoge"),
        age: 10,
    };

    p2.age = 100;

    println!("2: 変更したageフィールド {}", p2.age);

    p2 = Person {
        name: String::from("bar"),
        age: 20
    };

    println!("2: 再代入したp2インスタンス {:?}", p2);

    // 型が同じであれば、..で他のインスタンスのフィールド値を代入できる（スプレッド演算子みたいなもの）

    let p3 = Person {
        ..p1
    };

    println!("3: p3インスタンス {:?}", p3);

    let color = ColorRGB(255, 108, 108);
    println!("4: タプル構造体の値 {} {} {}", color.0, color.1, color.2);

    // 関連関数からインスタンスを生成する
    // 関連関数には::でアクセスする
    let p4 = Person::new(String::from("masato"), 36);

    // メソッドには.でアクセスする
    p4.greeting();

}
