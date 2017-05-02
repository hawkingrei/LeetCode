class Solution(object):
    def singleNumber(self, nums):
        ans = 0
        for n in nums:
            ans = ans ^ n
        return ans
