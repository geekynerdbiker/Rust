trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}, ({})", self.headline, self.author, self.location)
    }
}

struct Message {
    username: String,
    content: String,
}

impl Summary for Message {
    fn summarize(&self) -> String {
        format!("[New Message] {} ({})", self.content, self.username)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("The Rust Programming!"),
        location: String::from("California"),
        author: String::from("Jacob"),
    };

    let message = Message {
        username: String::from("Jacob An"),
        content: String::from("I like to study rust-lang."),
    };

    println!("{}", NewsArticle.summarize());
    println!("{}", Message.summarize());
}