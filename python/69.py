class Solution(object):
    def mySqrt(self, x):
    	left = 0
    	right = x
    	while left <= right:
    		mid = (left + right) /2
    		sq = mid * mid
    		if sq == x :
    			return mid
    		elif sq <x :
    			left = mid + 1
    		else:
    			right = mid -1 
    	return right

if __name__ == '__main__':
	sol = Solution()
	print sol.mySqrt(4)
	print sol.mySqrt(5)
