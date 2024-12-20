struct Solution; // declare a utility class that only contains static methods

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k: usize = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[k] = nums[i];
                k += 1;
            }
        }
        k as i32
    }
}

fn main() {
    let mut nums = vec![3, 2, 2, 3];
    let val = 3;
    let result = Solution::remove_element(&mut nums, val);
    println!("result: {}", result);
    println!("nums: {:?}", nums);
}