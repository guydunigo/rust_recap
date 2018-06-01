// ------------------------------
// OOP and rust
pub fn run() {
    // ------------------------------
    // Object traits
    {
        trait Draw {
            fn draw(&self);
        }

        struct Screen {
            components: Vec<Box<Draw>>,
        }

        impl Screen {
            fn run(&self) {
                for component in self.components.iter() {
                    component.draw();
                }
            }
        }

        struct Button {}
        impl Draw for Button {
            fn draw(&self) {
                println!("Button drawn !");
            }
        }

        struct Slider {}

        impl Draw for Slider {
            fn draw(&self) {
                println!("Slider drawn !");
            }
        }

        let a: Vec<Box<Draw>> = vec![
            Box::new(Button {}),
            Box::new(Slider {}),
            Box::new(Button {})
        ];
        let screen = Screen { components: a };
        screen.run();
    }
}

// ------------------------------
// Example

struct Post {
    content: String,
    state: Option<Box<State>>,
}

impl Post {
    fn new() -> Post {
        Post {
            content: String::new(),
            state: Some(Box::new(Draft {})),
        }
    }
    fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self.content)
    }
    fn add_text(&mut self, text_to_add: &str) {
        if let Some(state) = &self.state {
            state.add_text(&mut self.content, text_to_add);
        }
    }
    fn request_review(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }
    fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve());
        }
    }
}

trait State {
    fn content<'a>(&self, text: &'a str) -> &'a str {
        ""
    }
    fn add_text(&self, text: &mut String, text_to_add: &str) {
    }
    fn request_review(self: Box<Self>) -> Box<State>;
    fn approve(self: Box<Self>) -> Box<State>;
}

struct Draft {}
impl State for Draft {
    fn add_text(&self, text: &mut String, text_to_add: &str) {
        text.push_str(text_to_add);
    }
    fn request_review(self: Box<Self>) -> Box<State> {
        Box::new(Reviewing {})
    }
    fn approve(self: Box<Self>) -> Box<State> {
        self
    }
}

struct Reviewing {}
impl State for Reviewing {
    fn approve(self: Box<Self>) -> Box<State> {
        Box::new(Approved {})
    }
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }
}

struct Approved {}
impl State for Approved {
    fn content<'a>(&self, text: &'a str) -> &'a str {
        text
    }
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<State> {
        self
    }
}

#[test]
fn example_oop() {
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
