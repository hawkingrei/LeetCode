class Solution(object):
    def maxProfit(self, prices):
    	if not prices:
    		return 0
    	minprices = prices[0]
    	maxprofit = 0
    	for n in range(1,len(prices)):
    		if minprices > prices[n]:
    			minprices = prices[n]
    			continue
    		tmp = prices[n] - minprices
    		if 	tmp > maxprofit:
    			maxprofit = tmp
    	return maxprofit

if __name__ == '__main__':
	sol = Solution()
	print sol.maxProfit([7, 1, 5, 3, 6, 4])


