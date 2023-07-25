// todo: recommended challenges

// -  Add a reject method that changes the post’s state from PendingReview back to Draft.
// - Require two calls to approve before the state can be changed to Published.
// - Allow users to add text content only when a post is in the Draft state.
// Hint: have the state object responsible for what might change about the content,
// but not responsible for modifying the Post.

// - Implement states and behaviors "the Rust way" (shown in the last chapter sections), with types for different states
// - Do the first three challenges here following "the Rust way"

// encapsulation through a public struct and private fields
// as well as public methods to access the private fields
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// polymorphism through traits and trait objects
pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("drawing button");
        println!("width: {}", self.width);
        println!("height: {}", self.height);
        println!("label: {}", self.label);
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("drawing select box");
        println!("width: {}", self.width);
        println!("height: {}", self.height);
        println!("options: {:?}", self.options);
    }
}

// Box<dyn Draw> is a trait object
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// trait objects are different from generics with trait bounds
// because they are compiled to use dynamic dispatch (via virtual table)
// meaning the collection can have different concrete types
// instead of being a homogeneous collection of one concrete type
// this trade-off is for flexibility in favor performance
// faster compile times and smaller binary size, but slower runtime
// note: a trait object cannot be "downcasted" to a more concrete type
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

use oop::blog::Post;

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    // // type inference will not allow for the following
    // let components = vec![
    //     Box::new(SelectBox { /* .. */ }),
    //     Box::new(Button { /* .. */ }),
    // ];
    // let screen = Screen { components };
    // // the compiler loses that information at the point where components is defined
    // this can be fixed with the type annotation on components
    // or by  casting an element of the vector to a trait object (as Box<dyn Draw>)
    screen.run();

    // State Design Pattern - implemented in lib.rs
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
