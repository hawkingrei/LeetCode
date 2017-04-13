class Solution(object):
    def countBits(self, num):
    	dp = [ [ 0 for _ in range(len(bin(num))-2)] for _ in range(num+1) ]
    	result = [0]
    	for i in range(1,num+1):
    		add = True
    		dp[i] = dp[i-1]
    		for j in range(len(bin(num))-3,-1,-1):
    			if add:
    				if dp[i][j] == 1:
    					dp[i][j] = 0
    				else:
    					dp[i][j] = 1
    					add = False
    			else:
    				break
    		result.append(sum(dp[i]))
    	return result

if __name__ == '__main__':
	sol = Solution()
	print sol.countBits(10)

