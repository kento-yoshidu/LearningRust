struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    fn compare_area(&self, other: &Rectangle) -> bool {
        self.width * self.height > other.width * other.height
    }
}

fn double_value(a: i32) -> i32 {
    a * 2
}

fn greeting(name: &str) -> String {
    format!("Hello {} san", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_a_is_larger() {
        let a = Rectangle {
            width: 5,
            height: 5
        };

        let b = Rectangle {
            width: 2,
            height: 2
        };

        assert!(a.compare_area(&b));
    }

    #[test]
    fn test_a_is_smaller() {
        let a = Rectangle {
            width: 5,
            height: 5
        };

        let b = Rectangle {
            width: 10,
            height: 10
        };

        assert!(!(a.compare_area(&b)));
    }

    #[test]
    fn test_double() {
        assert_eq!(6, double_value(3));
    }

    #[test]
    fn test_contains_name() {
        let res = greeting("kento");

        assert!(res.contains("kento"));
    }
}
