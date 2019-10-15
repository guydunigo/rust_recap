// ------------------------------
// Structs
pub fn run() {
    struct User {
        username: String,
        email: String,
        sing_in_count: u64,
        active: bool,
    }

    fn build_user(email: String, username: String) -> User {
        User {
            email, // don't have to repeat "email: email"
            username,
            active: true,
            sing_in_count: 1,
        }
    }

    let mut user = User {
        email: String::from("s@s.s"),
        username: String::from("s"),
        active: true,
        sing_in_count: 1,
    };
    user.email = String::from("a@a.a");

    let user = build_user(String::from("i@i.i"), String::from("i"));
    // Use values from another user to init the struct
    let user2 = User {
        email: String::from("2@2.2"),
        username: String::from("2"),
        ..user
    };

    // Tuple structs
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);

    // ------------------------------
    // Struct methods

    struct Rectangle {
        w: u32,
        h: u32,
    };

    impl Rectangle {
        fn area(&self) -> u32 {
            self.w * self.h
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
            (self.w > other.w) && (self.h > other.h)
        }
        // associated function
        fn square(size: u32) -> Rectangle {
            Rectangle { w: size, h: size }
        }
    }

    let r = Rectangle { w: 3, h: 2 };
    let r2 = Rectangle::square(3);
    println!("R(3,2) area : {}", r.area());
    println!("R(1,2) in R(3,2) ? {}", r.can_hold(&r2));
}

// ------------------------------
// Example

enum PostState {
    Draft,
    Reviewing,
    Approved,
}

struct Post {
    content: String,
    state: PostState,
}

impl Post {
    fn new() -> Post {
        Post {
            content: String::new(),
            state: PostState::Draft,
        }
    }
    fn add_text(&mut self, text: &str) {
        if let PostState::Draft = self.state {
            self.content.push_str(text);
        }
    }
    fn content(&self) -> &str {
        match self.state {
            PostState::Approved => &self.content,
            // PostState::Draft|PostState::Reviewing => "",
            _ => "",
        }
    }
    fn request_review(&mut self) {
        if let PostState::Draft = self.state {
            self.state = PostState::Reviewing;
        }
    }
    fn approve(&mut self) {
        if let PostState::Reviewing = self.state {
            self.state = PostState::Approved;
        }
    }
}

#[test]
fn example_structs() {
    let mut post = Post::new();
    let sentence = "I ate a salad for lunch today";

    post.add_text(sentence);
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.add_text(sentence);

    post.approve();
    assert_eq!(sentence, post.content());

    post.add_text(sentence);
    assert_eq!(sentence, post.content());
}
