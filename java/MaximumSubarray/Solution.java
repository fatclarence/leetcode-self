import java.util.HashMap;
import java.util.Arrays;

class Solution {
  public long maximumSubarraySum(int[] nums, int k) {
    HashMap<Integer, Integer> map = new HashMap<>();
    long maxSum = 0;
    long currentSum = 0;
    
    // Initialize first window
    for (int i = 0; i < k; i++) {
        currentSum += nums[i];
        map.put(nums[i], map.getOrDefault(nums[i], 0) + 1);
    }
    
    // Check if first window has all distinct elements
    if (map.size() == k) {
        maxSum = currentSum;
    }
    
    // Slide the window
    for (int i = k; i < nums.length; i++) {
        // Remove the leftmost element
        int leftmost = nums[i - k];
        currentSum -= leftmost;
        map.put(leftmost, map.get(leftmost) - 1);
        if (map.get(leftmost) == 0) {
            map.remove(leftmost);
        }
        
        // Add the new element
        currentSum += nums[i];
        map.put(nums[i], map.getOrDefault(nums[i], 0) + 1);
        
        // Update maxSum if we have k distinct elements
        if (map.size() == k) {
            maxSum = Math.max(maxSum, currentSum);
        }
    }
    
    return maxSum;
  }

  public static void main(String[] args) {
    Solution solution = new Solution();
    // int[] nums = {1, 5, 4, 2, 9, 9, 9};
    int[] nums = {14,7,7,7,12,7};
    int k = 2;
    long ans = solution.maximumSubarraySum(nums, k);
    System.out.println(ans);
  }
}