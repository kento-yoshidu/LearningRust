struct Person {
    id: i32,
    name: String
}

// デフォルト値を設定
impl Default for Person {
    fn default() -> Self {
        Self {
            id: 999,
            name: String::from("Default Name") }
    }
}

#[derive(Default, Debug)]
enum Subject {
    Japanese,
    #[default]
    English,
    Math,
}

pub fn run() {
    let s = String::default();

    println!("1: String型のデフォルト = {}", s);
    //=> 1: String型のデフォルト =

    let i = i32::default();

    println!("1: i32型のデフォルト = {}", i);
    //=> 1: i32型のデフォルト = 0

    let str: &str = Default::default();

    println!("1: 文字列スライスのデフォルト = {}", str);
    //=> 1: 文字列スライスのデフォルト =

    let person: Person = Default::default();

    println!("1: Personインスタンスのi32のデフォルト = {}, Stringのデフォルト = {}", person.id, person.name);
    //=> 1: Personインスタンスのi32のデフォルト = 999, Stringのデフォルト = Default Name

    let subject = Subject::default();

    println!("1: Subject列挙体のデフォルト = {:?}", subject);
    //=> 1: Subject列挙体のデフォルト = English
}
