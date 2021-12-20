pub trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub orther: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, By {} ({})", self.headline, self.location, self.orther)
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("Username {} Content {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
