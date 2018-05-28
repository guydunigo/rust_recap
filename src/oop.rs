// ------------------------------
// OOP and rust
pub fn run() {
    // ------------------------------
    // Template
    {
        trait Draw {
            fn draw(&self);
        }

        pub struct Screen {
            pub components: Vec<Box<Draw>>,
        }

        impl Screen {
            for component in self.components.iter() {
                component.draw();
            }
        }
        let a: Vec<Box<Draw>> = Vec::new();
    }
    // ------------------------------
    // Template
    {
    }
}
