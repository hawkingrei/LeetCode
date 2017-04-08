class Solution(object):
    def numDecodings(self, s):
	if not s: return 0

	dp = [0] * 3

	dp[0] = 1
	dp[1] = 1 if s[0] != '0' else 0

	for i in range(2,len(s) + 1):
		one_digit = int(s[i-1:i])
		two_digits = int(s[i-2:i])

		dp[i%3]=0
		if two_digits in range(10,27):
			dp[i % 3] += dp[(i-2)%3]
		if one_digit in range(1,10):
			dp[i % 3 ] += dp[(i-1)%3]
	return dp[len(s)%3]

if __name__ == '__main__':
	sol =Solution()
	print(sol.numDecodings('12'))