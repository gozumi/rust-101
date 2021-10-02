use chapter_10_2::notify;
use chapter_10_2::notify_long_form_implementation;
//use chapter_10_2::notify_with_multiple_traits;
use chapter_10_2::NewsArticle;
use chapter_10_2::ScientificPaper;
use chapter_10_2::Summary;
use chapter_10_2::Tweet;

fn main() {
    let news_article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Peterburgh, UK"),
        author: String::from("Ice T"),
        content: String::from(
            "The Peterburgh Penguins once again are the best \
             football team in the Premier League.",
        ),
    };

    let tweet = Tweet {
        username: String::from("Busta Rhymes"),
        content: String::from("This weather is awful in London right now!"),
        reply: false,
        retweet: false,
    };

    let scientific_paper = ScientificPaper {
        title: String::from("The great awakening!"),
        authors: String::from("Kwame Nkruma, Marcus Garvey"),
        content: String::from("This is a really interesting article."),
        institution: String::from("The University of Ghana"),
    };

    println!("New article available! {}", news_article.summarize());
    println!("New tweet available! {}", tweet.summarize());
    println!(
        "Scientific paper available! {}",
        scientific_paper.summarize()
    );
    println!("News article author: {}", news_article.summarize_author());
    println!(
        "Scientific paper authors: {}",
        scientific_paper.summarize_author()
    );
    println!("Tweet author: {}", tweet.summarize_author());

    notify(&tweet);
    notify(&news_article);
    notify(&scientific_paper);
    notify_long_form_implementation(&tweet);
    notify_long_form_implementation(&news_article);
    notify_long_form_implementation(&scientific_paper);
    //notify_with_multiple_traits(&tweet);
}
