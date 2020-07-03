#![allow(dead_code)]

trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }

    fn summarize_author(&self) -> String;
}

struct NewArticle {
    headline: String,
    location: String,
    author: String,
}

impl Summary for NewArticle {
    // Use default behaviour
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn notify(summary: &impl Summary) {
    println!("Breaking news! {}", summary.summarize());
    // println!("{}", summary.summarize_author());
}

fn notify_t<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    };

    let new_article = NewArticle {
        headline: String::from("FooBar"),
        author: String::from("baz_bla"),
        location: String::from("BR"),
    };

    notify(&tweet);
    notify(&tweet);
    println!("===========================");
    notify_t(&new_article);
    notify_t(&new_article);

    println!("===========================");

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
