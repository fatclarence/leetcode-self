// package java.ConcatArrays.Solution;
import java.util.Arrays;

class Solution {
  public int[] getConcatenation(int[] nums) {
      int n = nums.length;
      int[] ans = new int[2 * n];
      for (int i = 0; i < 2*n; i++) {
          ans[i] = nums[i % n];
      }

    return ans;
  }

  public static void main(String[] args) {
    Solution solution = new Solution();
    int[] nums = {1, 2, 3};
    int[] ans = solution.getConcatenation(nums);
    System.out.println(Arrays.toString(ans));
  }
}
