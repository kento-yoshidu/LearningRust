pub fn run() {
    println!("1: {}", 'A'.is_uppercase());
    //=> true

    println!("1: {}", 'a'.is_uppercase());
    //=> false

    // String型には生えていない
    // println!("🦀❌ {}", String::from("Hello").is_uppercase());

    // &strにも生えていない
    // println!("🦀❌ {}", "Hello".is_uppercase());

    // chars()にして回す
    /*
    let s = String::from("Hello World");

    s.chars().for_each(|c| {
        println!("{} = {}", c, c.is_uppercase());
        /*
            H = true
            e = false
            l = false
            l = false
            o = false
            = false
            W = true
            o = false
            r = false
            l = false
            d = false
        */
    });

    // filterを使って大文字をカウントする
    let count = s.chars().filter(|c| {
        c.is_uppercase()
    }).count();

    println!("{}", count);
    //=> 2
    */

    let s2 = "Hello World";

    s2.chars().for_each(|c| {
        println!("{} = {}", c, c.is_uppercase());
        /*
            H = true
            e = false
            l = false
            l = false
            o = false
            = false
            W = true
            o = false
            r = false
            l = false
            d = false
        */
    });

    let count2 = s2.chars().filter(|c| {
        c.is_uppercase()
    }).count();

    println!("{}", count2);
    //=> 2
}
