fn main() {
    let mut nums = Vec::from([1, 8, 49, 22, 1]);
    nums.push(99);
    println!("{}", get_biggest(&nums));
    let range = get_range(&nums);
    println!("{}", range);
}

fn get_biggest(vec: &Vec<i32>) -> i32 {
    let mut largest = 0;
    for num in vec {
        if *num > largest {
            largest = *num
        }
    }
    largest
}

fn get_range(vec: &Vec<i32>) -> i32 {
    let mut copy = vec.clone();
    copy.sort();
    copy.last().unwrap() - copy.first().unwrap()
}
