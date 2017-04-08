from sys import maxint
class Solution(object):
    def maxProfit(self, k, prices):
    	if not prices: return 0
    	n = len(prices)

    	if k>n/2:
    		self._get_max_profit(prices)

    	profit = [[[-maxint for _ in range(2)] for _ in range(k+1)] for _ in range(2)]
    	profit[0][0][0], profit[0][0][1] = 0, - prices[0]
    	res = 0
    	for i in range(1,n):
    		x,y = i % 2, (i - 1) % 2
    		for j in range(k+1):
    			profit[x][j][0] = profit[y][j][0] if j == 0 else max(profit[y][j][0],profit[y][j-1][1]+prices[i])
    			profit[x][j][1] = max(profit[y][j][1],profit[y][j][0] - prices[i])
    			res = max(res,profit[x][j][0])
    	return res

    def _get_max_profit(self,prices):
    	res = 0;
    	for i in range(1,len(prices)):
    		if prices[i] - prices[i-1] > 0:
    			res += prices[i] - prices[i-1]
    	return res


class Solution(object):
    def maxProfit(self, k, prices):
        """
        :type k: int
        :type prices: List[int]
        :rtype: int
        """
    
        if not prices:
            return 0
        
        if k >= len(prices) // 2:
            return sum(
                x - y
                for x, y in zip(prices[1:], prices[:-1])
                if x > y)
        
        
        profits = [0]*len(prices)
        for j in range(k):

            preprofit = 0
            for i in range(1,len(prices)):
            
                profit = prices[i] - prices[i-1]
                preprofit = max(preprofit+profit, profits[i])
                profits[i] = max(profits[i-1], preprofit)
    
        return profits[-1]