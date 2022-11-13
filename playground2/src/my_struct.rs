use std::str::ParseBoolError;

pub fn run (){
    /*
     * 構造体にはCopyトレイトが実装されていないため、デフォルトでmoveセマンティクスが適用される。
     */

    // https://users.rust-lang.org/t/cant-derive-copy-because-of-string/18665/6

    /* 構造体のフィールドに別の構造体を置ける */

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8
    }

    #[derive(Debug)]
    struct Parents {
        father: Person,
        mother: Person
    }

    let kento = Person {
        name: String::from("kento"),
        age: 10
    };
    let keiko = Person {
        name: String::from("keiko"),
        age: 20
    };

    let sato = Parents {
        father: kento,
        mother: keiko
    };

    // kento, keikoの所有権は移動する
    // println!("{:?}", sato);

    // 所有権がないためエラー
    // println!("{:?}", kento);
    // println!("{:?}", keiko);

    /* ****************************** */
    let takashi = Person {
        name: String::from("takashi"),
        age: 52
    };

    let yukina = Person {
        name: String::from("yukina"),
        age: 43
    };

    // ライフタイムパラメーターを明示し、参照にする
    #[derive(Debug)]
    struct Parents2<'a, 'b> {
        father: &'a Person,
        mother: &'b Person
    }

    // メソッドも定義する
    impl<'a, 'b> Parents2<'a, 'b> {
        fn new(father: &'a Person, mother: &'b Person) -> Parents2<'a, 'b> {
            Parents2 { father, mother }
        }
    }

    let sato = Parents2::new(&takashi, &yukina);

    println!("{:?}", sato);
}
