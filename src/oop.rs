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
            components: Vec<Box<dyn Draw>>,
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

        let a: Vec<Box<dyn Draw>> = vec![
            Box::new(Button {}),
            Box::new(Slider {}),
            Box::new(Button {}),
        ];
        let screen = Screen { components: a };
        screen.run();
    }
}

// ------------------------------
// Example

struct Post {
    content: String,
    state: Option<Box<dyn State>>,
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
    fn add_text(&self, text: &mut String, text_to_add: &str) {}
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}
impl State for Draft {
    fn add_text(&self, text: &mut String, text_to_add: &str) {
        text.push_str(text_to_add);
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(Reviewing {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct Reviewing {}
impl State for Reviewing {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Approved {})
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct Approved {}
impl State for Approved {
    fn content<'a>(&self, text: &'a str) -> &'a str {
        text
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
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

// New implementation that causes compile time error if we call the functions when we are not
// supposed to.

struct Post2 {
    content: String,
}

struct Draft2 {
    content: String,
}

impl Post2 {
    fn new() -> Draft2 {
        Draft2 {
            content: String::new(),
        }
    }

    fn content(&self) -> &str {
        &self.content
    }
}

impl Draft2 {
    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    fn request_review(self) -> Reviewing2 {
        Reviewing2 {
            content: self.content,
        }
    }
}

struct Reviewing2 {
    content: String,
}

impl Reviewing2 {
    fn approve(self) -> Post2 {
        Post2 {
            content: self.content,
        }
    }
}

#[test]
fn example_oop_2() {
    let mut post = Post2::new();
    let sentence = "I ate a salad for lunch today";

    post.add_text(sentence);
    // assert_eq!("", post.content());

    let post = post.request_review();
    // assert_eq!("", post.content());

    // post.add_text(sentence);

    let post = post.approve();
    // assert_eq!(sentence, post.content());

    // post.add_text(sentence);
    assert_eq!(sentence, post.content());
}
