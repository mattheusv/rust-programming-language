fn main() {
    println!("first -> {}", ping_latin(String::from("first")));
    println!("apple -> {}", ping_latin(String::from("apple")));
}

fn ping_latin(word: String) -> String {
    let vowels = vec!["a", "e", "i", "o", "u"];

    let first_letter = &word[0..1];
    let rest_of_word = &word[1..];

    if vowels.contains(&first_letter) {
        return format!("{}-hay", word);
    }

    format!("{}-{}ay", rest_of_word, first_letter)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ping_latin_vowel() {
        let word = ping_latin(String::from("apple"));

        assert_eq!(word, String::from("apple-hay"))
    }

    #[test]
    fn ping_latin_consonant() {
        let word = ping_latin(String::from("first"));

        assert_eq!(word, "irst-fay");
    }
}
