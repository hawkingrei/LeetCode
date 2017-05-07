public class Solution {
    public int[] maxSlidingWindow(int[] nums, int k) {
    	if (nums == null || nums.length == 0 ) return new int[0];

		Queue<Integer> pq = new PriorityQueue<Integer>(k ,Collections.reverseOrder());

		int[] result = new int[nums.length-k+1];

		for (int i = 0;i < nums.length;++i){
			if (i >= k) {
				pq.remove(nums[i-k]);
			}
			pq.add(nums[i]);
			if (i >= k -1) {
				result[i - (k - 1)] = pq.peek();
			}
		}
		return result;
	}
}