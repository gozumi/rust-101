use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

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
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct ScientificPaper {
    pub title: String,
    pub authors: String,
    pub content: String,
    pub institution: String,
}

impl Summary for ScientificPaper {
    fn summarize_author(&self) -> String {
        format!("@{}", self.authors)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_long_form_implementation<T: Summary>(item: &T) {
    println!(
        "Breaking news via long form implementation! {}",
        item.summarize()
    );
}

pub fn notify_with_multiple_traits(item: &(impl Summary + Display)) {
    println!("Breaking news via multiple traits! {}", item.summarize());
}
