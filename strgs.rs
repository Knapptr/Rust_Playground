fn main() {
    let mut answer = String::new();
    std::io::stdin()
        .read_line(&mut answer)
        .expect("Error reading line");
    answer = answer.trim().to_string();
    let ified = sp_gar(&answer);
    println!("{ified}");
}

fn sp_gar(str: &String) -> String {
    let mut garified = String::new();
    for (idx, char) in str.char_indices() {
        match idx % 2 {
            0 => garified.push(char.to_ascii_lowercase()),
            _ => garified.push(char.to_ascii_uppercase()),
        }
    }
    garified
}
