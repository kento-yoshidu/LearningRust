trait CalcArea {
    fn calc_area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64
}

impl CalcArea for Rectangle {
    fn calc_area(&self) -> f64 {
        self.width * self.height
    }
}

struct RightTriangle {
    width: f64,
    height: f64
}

impl CalcArea for RightTriangle {
    fn calc_area(&self) -> f64 {
        self.width * self.height * 0.5
    }
}

fn area<T: CalcArea>(x: &T) -> f64 {
    x.calc_area()
}

trait CalcLength {
    fn calc_length(&self) -> f64;
}

struct Line {
    length: f64
}

impl CalcLength for Line {
    fn calc_length(&self) -> f64 {
        self.length
    }
}

impl CalcLength for Rectangle {
    fn calc_length(&self) -> f64 {
        (self.width + self.height) * 2.0
    }
}

impl CalcLength for RightTriangle {
    fn calc_length(&self) -> f64 {
        self.width * self.height + (self.width.powi(2) + self.height.powi(2)).sqrt()
    }
}

fn length<T: CalcLength>(x: &T) -> f64 {
    x.calc_length()
}

pub fn run() {
    let rect = Rectangle { width: 1.0, height: 2.0 };
    println!("{}", area(&rect));
    println!("{}", length(&rect));

    let tria = RightTriangle { width: 1.0, height: 2.0 };
    println!("{}", area(&tria));
    println!("{}", length(&tria));

    let line = Line { length: 5.0 };
    println!("{}", length(&line));
    // println!("{}", area(&line));
}

/*
pub fn run() {
    pub trait Shape {
        fn calc_area(&self) -> f64;
        fn calc_perimiter(&self) -> f64;
        fn do_something();
        fn default_somethinf(&self) -> &str {
            "This is default method."
        }
    }

    pub struct Rectangle {
        pub width: f64,
        pub height: f64
    }

    impl Shape for Rectangle {
        fn calc_area(&self) -> f64 {
            self.width * self.height
        }

        fn calc_perimiter(&self) -> f64 {
            self.width * 2.0 + self.height * 2.0
        }

        fn do_something() {
            println!("This is Rectangle function.")
        }
    }

    pub struct Circle {
        pub radius: f64
    }

    impl Shape for Circle {
        fn calc_area(&self) -> f64 {
            self.radius * self.radius * std::f64::consts::PI
        }

        fn calc_perimiter(&self) -> f64 {
            self.radius * 2.0 * std::f64::consts::PI
        }

        fn do_something() {
            println!("This is Circle function");
        }
    }

    let rect = Rectangle {
        width: 4.0,
        height: 5.0
    };

    println!("rect {}", rect.calc_area());

    fn double_area(shape: &impl Shape) -> f64 {
        shape.calc_area() * 2.0
    }

    println!("{}", double_area(&rect));
}
*/
