struct Solution; // declare a utility class that only contains static methods

impl Solution {
    // takes in an array of integers and returns the largest sum of
    // a contiguous subarray
    pub fn max_sub_array(nums: &Vec<i32>) -> i32 {
        let mut max_sum = i32::MIN;

        for i in 0..nums.len() {
            let mut curr_sum = 0;

            for j in i..nums.len() {
                // Note: for integers don't need to use reference
                // since i32 primitive type implements Copy trait
                // and is as efficient and cleaner to work with than using
                // references.
                let val = nums[j];
                curr_sum += val;
                max_sum = max_sum.max(curr_sum);
            }
        }
        max_sum
    }
}

fn main() {
    let nums = vec![4, -1, 2, -7, 3, 4];
    let result = Solution::max_sub_array(&nums);
    println!("Nums: {:?}", nums);
    println!("Result: {}", result);
}
