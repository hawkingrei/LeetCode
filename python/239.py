class Solution(object):
	def maxSlidingWindow(self, nums, k):
		if not nums: return []

		window_index = []
		res = []
		for i, x in enumerate(nums):
			if i >= k and windows_index[0] <= i - k:
				window_index.pop(0)
			while window_index and nums[window_index[-1]] <= x:
				window_index.pop()

			window_index.append(i)

			if i >= k - 1:
				res.append(nums[window_index[0]])
		return res