use std::collections::HashMap;

// problems
// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.

// [4, 99, 20, 28, 1, 28]
// [1, 4, 20, 28, 28, 99] median: 28
// [1, 4, 20, 28, 28, 99] mode: 28
// [1, 4, 20, 28, 28, 99] mean: 30
//Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

//Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
fn main() {
    let vec = Vec::from([4, 99, 20, 28, 1, 28]);
    println!("{:?}", get_stats(vec));
}

#[derive(Debug)]
struct StatTuple(i32, i32, i32); //mean,median,mode
                                 //
fn get_stats(list: Vec<i32>) -> StatTuple {
    //create vector from list
    let mut vec = Vec::from(list);
    //sort vector
    vec.sort();
    //get stats
    let mean = get_mean(&vec);
    let median = get_median(&vec);
    let mode = get_mode(&vec);
    StatTuple(mean, median, mode)
}

fn get_mean(sorted_vec: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for num in sorted_vec {
        sum += num;
    }
    sum / sorted_vec.len() as i32
}

fn get_median(sorted_vec: &Vec<i32>) -> i32 {
    let count = sorted_vec.len();
    if count % 2 == 0 {
        return get_mean(&Vec::from([
            sorted_vec[count / 2],
            sorted_vec[count / 2 + 1],
        ]));
    }
    sorted_vec[count / 2]
}

fn get_mode(vec: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    // count numbers in list
    for num in vec {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    let mut most_frequent = (i32::MIN, i32::MIN);
    for (key, value) in map {
        if value > most_frequent.1 {
            most_frequent = (*key, value);
        }
    }
    most_frequent.0
}
