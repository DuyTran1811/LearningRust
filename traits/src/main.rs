use traits::{NewArticle, Tweet};

fn main() {
    let _article = NewArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        orther: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    println!("New article available! {:?} ", _article);

    let _tweets = Tweet {
        username: String::from("D.Tran"),
        content: String::from("I doing learn Rust"),
        reply: true,
        retweet: true,
    };

    println!("Is new Tweet {:?}", _tweets)
}
