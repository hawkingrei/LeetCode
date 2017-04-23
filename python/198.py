class Solution(object):
    def rob(self, nums):
        best0 = 0
        best1 = 0
        temp = 0
        for i in range(len(nums)):
            temp = best1 + nums[i]
            best1 = best0
            best0 = max(best0, temp)
        
        return max(best0, best1)



class Solution(object):
    def rob(self, nums):
    	if len(nums)<=1:
    		return sum(nums)
    	dp = [ 0 for _ in range(len(nums))]
    	dp[0] = nums[0]
        dp[1] = max(nums[0],nums[1])
    	for i in range(2,len(nums)):
    		dp[i] = max(dp[i-1],dp[i-2]+nums[i])
        return dp[-1]