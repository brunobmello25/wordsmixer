use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Config {
    pub words: Vec<String>,
}

impl Config {
    pub fn load(mut args: impl Iterator<Item = String>) -> Self {
        args.next();

        let words: Vec<String> = args.collect();

        return Config { words };
    }
}

pub fn mix_words(words: &Vec<String>) -> Vec<String> {
    let result: Vec<String> = words.iter().map(|word| mix_word(word)).collect();

    return result;
}

fn mix_word<'a>(word: &'a str) -> String {
    if word.len() < 3 {
        return word.to_string();
    }

    let mut chars = word.chars();
    let first = chars.next().unwrap();
    let last = chars.next_back().unwrap();
    let mut middle: Vec<char> = chars.collect();

    middle.shuffle(&mut thread_rng());

    return format!(
        "{}{}{}",
        first,
        middle.into_iter().collect::<String>(),
        last
    );
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn should_empty_when_empty() {
        let result = mix_word("");

        assert_eq!(result, "");
    }

    #[test]
    fn should_return_same_when_word_is_small() {
        let result = mix_word("ab");

        assert_eq!(result, "ab");
    }

    #[test]
    fn should_return_mixed_when_word_is_big() {
        let result = mix_word("abcdefghij");

        assert_eq!(result.chars().next().unwrap(), 'a');
        assert_eq!(result.chars().next_back().unwrap(), 'j');
        assert_ne!(get_middle(&result), "bcdefghi");
        assert_eq!(sort_word(get_middle(&result)), "bcdefghi");
    }

    fn get_middle(word: &String) -> &str {
        let mut chars = word.chars();
        chars.next();
        chars.next_back();
        return chars.as_str();
    }

    fn sort_word<'a>(word: &'a str) -> String {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort_by(|a, b| a.cmp(b));
        return chars.into_iter().collect::<String>();
    }
}
