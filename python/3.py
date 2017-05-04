class Solution(object):
    def lengthOfLongestSubstring(self, s):
        ans = 0 
        last = {}
        left = 0
        for n in range(len(s)):
            if s[n] in last and last[s[n]] >= left:
                left = last[s[n]] + 1
            last[s[n]] = n
            ans =max(ans,n-left+1)
        return ans
            

