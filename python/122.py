class Solution(object):
    def maxProfit(self, prices):
        if not prices or len(prices) == 1:
    		return 0
        result = [[0 for _ in range(2)] for _ in range(len(prices)) ] 
    	if prices[0] < prices[1]:
    	    result[0][0] = - prices[0]
    	    result[0][1] = 1
    	for n in range(1,len(prices)):
            if n == len(prices) - 1 :
                if result[n-1][1] == 1:
                    result[n][0] = result[n-1][0] + prices[n]
                    result[n][1] = 0
                    break
                result[n][0] = result[n-1][0]
                result[n][1] = result[n-1][1]
                break
            if result[n-1][1] == 0:
                if prices[n] < prices[n+1]:
                    result[n][0] = result[n-1][0] - prices[n]
                    result[n][1] = 1
                    continue
            else:
                if prices[n] > prices[n+1]:
                    result[n][0] = result[n-1][0] + prices[n]
                    result[n][1] = 0
                    continue
            result[n][0] = result[n-1][0]
            result[n][1] = result[n-1][1]
    	return result[len(prices)-1][0]

if __name__ == '__main__':
    sol = Solution()
    print sol.maxProfit([4,5,6,8,7,4])
    print sol.maxProfit([9])
