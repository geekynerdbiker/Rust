trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
// impl Trait syntax
fn notify1(item: impl Summary) {
    println!("Nofity1: {}", item.summarize());
}

// Trait Border syntax
fn notify2<T: Summary> (item: T) {
    println!("Notify2: {}", item.summarize());
}

fn notify3(item1: impl Summary, item2: impl Summary) {
    println!("[Notify3]\nitem1: {}, item2: {}", item1.summarize(), item2.summarize());
}

fn notify4<T: Summary> (item1: T, item2: T) {
    println!("[Notify3]\nitem1: {}, item2: {}", item1.summarize(), item2.summarize());
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    let article2 = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    //notify1(article);
    //notify2(tweet);
    //notify3(article, tweet);
    //notify3(article, article2);
    //notify4(article, article2);
}