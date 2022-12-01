fn main() {
    println!("{}", alternate_case(&"hellotheregoodsir".to_string()))
}

fn alternate_case(s: &String) -> String {
    let mut new_st = String::new();
    for (index, char) in s.char_indices() {
        match index % 2 {
            0 => new_st += char.to_uppercase().to_string().as_str(),
            _ => new_st += char.to_lowercase().to_string().as_str(),
        }
    }
    new_st
}
