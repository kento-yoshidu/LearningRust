#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn create(width: u32, height: u32) -> Self {
        Self{ width, height }
    }

    fn area(&self) {
        println!("{}", self.width * self.height);
    }
}

pub fn run() {
    let user1 = User {
        email: String::from("test@test"),
        username: String::from("kento"),
        active: true,
        sign_in_count: 0
    };

    let user2 = build_user(String::from("hoge@hoge"), String::from("keiko"));

    println!("{:#?}", user2);

    let rect = Rectangle::create(20, 20);

    println!("{:#?}", rect);

    rect.area();

}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 0,
        active: true
    }
}
