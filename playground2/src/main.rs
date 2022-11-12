mod my_print;
mod my_struct;
mod my_enum;

fn my_print<T: std::fmt::Display>(str: &T) {
    println!("{}", str);
}

fn myclear(x: &mut String) {
    println!("clear前のx = {}", x);

    x.clear();

    println!("clear後のx = {}", x);
}

/*
fn retrun_hello() -> &String {
    let s = String::from("Hello");
    &s
}
*/

fn pick1(x: &[i32], end: usize) -> &[i32] {
    &x[..end]
}

fn pick2<'a, 'b>(x: &'a [i32], y: &'b [i32], end: usize) -> (&'a [i32], &'b [i32]) {
    (&x[..end], &y[..end])
}

fn main() {
    let s = String::from("Hello");

    let ref_s = &s;
    let ref_s2 = &s;

    /*
    my_print(ref_s);
    my_print(ref_s2);
    */

    let mut s = String::from("Hello");
    println!("{}", s);

    let ref_s = &mut s;
    // 同時に2つ以上のミュータブルな参照は存在できない
    // let ref_s2 = &mut s;

    myclear(ref_s);

    let ref_s = &mut s;
    myclear(ref_s);

    let ref_s2 = &mut s;
    myclear(ref_s2);

    /* ************************************* */

    let mut x = 1;
    let x_ref = &x;

    // xの参照（x_ref）が生きている間の値の変更はできない
    // x = 2;
    println!("{}", x_ref);

    /* ************************************* */

    let x;
    {
        let y = 1;
        x = &y;
    }
    // println!("{}", x);

    /* ************************************* */

    let v1 = [1, 2, 3, 4, 5];
    let p = pick1(&v1, 2);

    for ss in p {
        println!("{}", ss);
    }

    /* ************************************* */

    let v1 = [1, 2, 3, 4, 5];
    let v2 = [6, 7, 8];

    let p = pick2(&v1, &v2, 2);

    for ss in p.0 {
        println!("ライフサイクルパラメーター{}", ss);
    }

    for ss in p.1 {
        println!("ライフサイクルパラメーター{}", ss);
    }

    /* ************************************* */

    // &'static str型
    /*
     * 実行初期に静的領域に配置される。
     * ライフタイムパラメーターは'static。
     */
    let s = "Hello World";

    println!("{}", &s[1..5]);

    /* ************************************* */

    struct Person {
        name: String,
        age: u8
    }

    let kento = Person {
        name: String::from("kento"),
        age: 10
    };

    println!("{}, {}", kento.name, kento.age);

    let mut keiko = Person {
        name: String::from("keiko"),
        age: 18
    };

    keiko.age = 50;

    println!("{}, {}", keiko.name, keiko.age);

    // タプル構造体

    struct ColorRGB(u32, u32, u32);

    let color = ColorRGB(255, 255, 0);

    println!("{}, {}, {}", color.0, color.1, color.2);

    #[derive(Debug)]
    struct Person2 {
        name: String,
        age: u8
    }

    impl Person2 {
        fn new(name: String, age: u8) -> Person2 {
            Person2 { name, age }
        }

        fn age_incr(&self, incr: u8) -> u8 {
            self.age + incr
        }

        fn age_incr_replace(&mut self, incr: u8) {
            self.age += incr;
        }

        // 所有権が移動する
        fn owner(self) {
            println!("{:?}", self);
        }
    }

    let kento = Person2::new(String::from("kento"), 10);

    println!("{:?}", kento);

    println!("{}", kento.age_incr(10));

    let mut kento = Person2::new(String::from("kento"), 10);

    kento.age_incr_replace(30);

    kento.owner();

    // my_struct::run();
    // my_enum::run();
}

// https://zenn.dev/ucwork/articles/6de5c9c2257f2d
// https://zenn.dev/tokeiya3/articles/8f9a9b9069c0db

