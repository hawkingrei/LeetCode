class Solution(object):
	def getOptimizedState(self,dp,nums,i):
		maxlength = 0
		for j in range(i):
			if (nums[j]< nums[i]):
				maxlength = max(maxlength,dp[j]+1)
		return maxlength


	def lengthOfLIS(self, nums):
		if (nums == None or len(nums) == 0 ):
			return 0
		res = 0
		dp =[]
		for n in range(len(nums)):
			dp.append(0)
        
		for i in range(len(nums)):
			dp[i]  = self.getOptimizedState(dp,nums,i)
			res =max(res,dp[i])
		return res + 1
        
        
    

if __name__ == '__main__':
	a = Solution()
	print a.lengthOfLIS([10, 9, 2, 5, 3, 7, 101, 18])
	print a.lengthOfLIS([0])