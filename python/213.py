class Solution(object):
    def rob(self, nums):
        if len(nums)<=1:
            return sum(nums)
        dp = [ 0 for _ in range(len(nums))]
        dp[0] = nums[0]
        dp[1] = max(nums[0],nums[1])
        for i in range(2,len(nums)):
            dp[i] = max(dp[i-1],dp[i-2]+nums[i])
        answer = dp[-2]
        dp[0] = 0
        dp[1] = nums[1]
        for i in range(2,len(nums)):
            dp[i] = max(dp[i-1],dp[i-2]+nums[i])
        return max(dp[-1],answer)

if __name__ == '__main__':
    sol =Solution()
    print sol.rob([1,1,1])