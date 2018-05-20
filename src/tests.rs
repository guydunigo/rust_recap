#![allow(dead_code)]

pub fn add_two(x: i64) -> i64 {
    x + 2
}

pub fn pgcd(a: i64, b: i64) -> i64 {
    let min = if a < b { a } else { b };
    for i in (1..min+1).rev() {
        if (a % i == 0) && (b % i == 0) {
            return i
        }
    }

    1
}

pub fn can_hold<T>(dest: &[T], src: &[T]) -> bool
{
    dest.len() >= src.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        assert!(can_hold(&[4,4,4,4],&[2,2]));

        assert_eq!(pgcd(24, 12), 12);
        assert_ne!(pgcd(12, 24), 30);

        #[derive(PartialEq, Debug)]
        struct AssertEquality {
            x: i64,
            y: String
        }

        let ae0 = AssertEquality { x: 42, y: String::from("Salut") };
        let ae1 = AssertEquality { x: 42, y: String::from("Salut") };

        assert_eq!(ae0, ae1, "Error message to help understand the test failure.");
    }

    #[test]
    #[should_panic]
    fn it_doesnt() {
        panic!("Haha it fails !");
    }

    #[test]
    #[ignore]
    #[should_panic(expected = "Don't panic !")]
    fn it_neither() {
        panic!("Don't panic !");
    }
}
