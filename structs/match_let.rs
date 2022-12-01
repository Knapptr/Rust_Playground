#[allow(dead_code)]

fn main() {
    let resp = ret_option();
    // match syntax
    match resp {
        Some(num) => println!("The number is: {}", num),
        None => println!("No number given"),
    };
    // if let syntax
    if let Some(num) = resp {
        println!("The number is: {}", num);
    } else {
        println!("No number given");
    }
}

// set return value by commenting out/in
fn ret_option() -> Option<i32> {
    Some(21)
    // None
}
