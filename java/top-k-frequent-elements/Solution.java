import java.util.HashMap;
import java.util.PriorityQueue;

class Solution {
    class Pair<T> implements Comparable<Pair<T>> {
        private T first;
        private int second;

        Pair(T first, int second) {
            this.first = first;
            this.second = second;
        }

        // Need public and @Override
        @Override
        public int compareTo(Pair<T> other) {
            // return a negative number if other is larger
            return other.second - this.second;
        }

        public T getFirst() { return first; }
        public int getSecond() { return second; }
    }

    public int[] topKFrequent(int[] nums, int k) {
        // put elements into the hashmap, or update the count if already in hashmap
        HashMap<Integer, Integer> map = new HashMap<>(16, 0.9f);
        for (int i = 0; i < nums.length; i++) {
            map.put(nums[i], map.getOrDefault(nums[i], 0) + 1);
        }

        // Instantiate PQ
        PriorityQueue<Pair<Integer>> pq = new PriorityQueue<>();

        map.forEach((key, value) -> {
            pq.offer(new Pair<Integer>(key, value));
        });

        int[] values = new int[k];

        // poll from pq the k most frequent elements.
        for (int i = 0; i < k; i++) {
            Pair<Integer> pair = pq.poll();
            values[i] = pair.getFirst();
        }

        return values;
    }


    // LOCAL TESTING
    public static void main(String[] args) {
      Solution solution = new Solution();
      
      // Test case 1
      int[] nums1 = {1, 1, 1, 2, 2, 3};
      int k1 = 2;
      int[] result1 = solution.topKFrequent(nums1, k1);

      System.out.println("Test case 1:");
      System.out.println("Input: nums = [1,1,1,2,2,3], k = 2");
      System.out.print("Output: [");
      for (int i = 0; i < result1.length; i++) {
          System.out.print(result1[i]);
          if (i < result1.length - 1) System.out.print(", ");
      }
      System.out.println("]");
    }
}