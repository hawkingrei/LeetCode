class Solution(object):
    def isMatch(self, s, p):
        dp = [ [ False for _ in range(len(p) + 1) ] for _ in range(len(s) + 1) ]
        dp[0][0] = True
        for j in range(1,len(p)+1):
            if p[j - 1] == "*":
                dp[0][j] = dp[0][j - 2]
        for i in range(1,len(s)+1):
            for j in range(1,len(p)+1):
                if p[j - 1] == '*':
                    dp[i][j] = dp[i][j - 2]
                    if s[i - 1] == p[j - 2] or p[j - 2] == ".":
                        dp[i][j] |= dp[i-1][j]
                else:
                    if s[i - 1] == p[j - 1] or p[j - 1] == ".":
                        dp[i][j] = dp[i-1][j-1]
        
        return dp[len(s)][len(p)]

if __name__ == '__main__':
    sol = Solution()
    print sol.isMatch("aaaaaa","aa*")
    print sol.isMatch("aa","a")