fn main() {
    let mut nums = vec![1, 82, 20, 102, 8];
    let largest_num = largest(&nums);
    println!("Nums: {:?}\nLargest: {}", nums, largest_num);
    nums.clear();
    println!("Nums: {:?}\nLargest: {}", nums, largest_num);
}

fn largest(nums: &Vec<i32>) -> i32 {
    let mut largest: i32 = nums[0];
    for num in nums {
        if *num > largest {
            largest = *num;
        }
    }
    largest
}
