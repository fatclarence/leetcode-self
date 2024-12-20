struct Solution;

impl Solution {
    // Returns the value of the max subarray sum in O(n)
    pub fn kadane(nums: &Vec<i32>) -> i32 {
        let mut max_sum = i32::MIN;
        let mut curr_sum = i32::MIN;

        for i in 0..nums.len() {
            // we always want to either take the current 
            // pointed value, 
            // or take the pointed val + previous sum
            curr_sum = curr_sum.max(0);
            curr_sum += nums[i];
            max_sum = max_sum.max(curr_sum);
        }

        max_sum
    }
}

fn main() {
    let nums = vec![4, -1, 2, -7, 3, 4];
    let result = Solution::kadane(&nums);
    println!("Nums: {:?}", nums);
    println!("Result: {}", result);
}
