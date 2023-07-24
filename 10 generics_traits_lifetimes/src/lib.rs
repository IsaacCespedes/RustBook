pub mod aggregator {
    // traits are like interfaces, can have multiple methods
    // we can’t implement external traits on external types.
    pub trait Summary {
        fn summarize(&self) -> String;

        // default implementation, can call non default methods also
        // only non default methods must be implemented
        fn default_summary(&self) -> String {
            String::from("(Read more...)")
        }
    }

    // if a trait will consist of only defaults, it can be implemented without any methods
    //   pub trait Summary {
    //     // default implementation
    //     fn default_summary(&self) -> String {
    //         String::from("(Read more...)")
    //     }
    // }
    // ...
    // impl Summary for Tweet {}

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
}
