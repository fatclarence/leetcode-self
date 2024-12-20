impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid =  (right + left).div_euclid(2);
            if (nums[mid] < target) {
                // on right side
                left = mid + 1;
            } else if (nums[mid] > target) {
                if mid == 0 {
                    break;
                }
                right = mid - 1;
            } else {
                return mid as i32;
            }
        }

        return -1;
    }
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let target = 3;
    let result = Solution::search(nums, target);
    println!("result: {}", result);
}