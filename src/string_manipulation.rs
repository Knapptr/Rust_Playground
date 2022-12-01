fn main() {
    let str = "Hello from the &str type 🦀";
    let string = String::from("Hello from the String type");
    println!("Str: {}\nString: {}", str, string);

    let last_char_str = get_last_char(&str);
    let last_char_string = get_last_char(&string);
    let first_word_str = get_first_word(str);

    let longest = get_longest_word(str);
    let longest_string = get_longest_word(&string);

    println!("Longest, string: {}", longest_string);

    println!("First Word, Str: {}", first_word_str);
    println!("Str: {}\nString: {}", str, string);
    println!(
        "Last Str: {}\nLast String: {}",
        last_char_str, last_char_string
    );
}

fn get_last_char(str: &str) -> String {
    str.trim()
        .chars()
        .last()
        .expect("something terrible happened.")
        .to_string()
}

fn get_first_word(str: &str) -> String {
    str.trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .to_string()
}

fn get_longest_word(str: &str) -> String {
    let words: Vec<&str> = str.trim().split_whitespace().collect();
    let mut longest = String::new();
    let mut longest_count: u8 = u8::MIN;
    for word in words {
        let count = word.chars().count();
        if count > longest_count as usize {
            longest_count = count as u8;
            longest = String::from(word)
        }
        println!("Word in longest: {}, {}", word, word.chars().count());
    }
    longest
}


