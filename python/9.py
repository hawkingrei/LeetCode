class Solution(object):
    def isPalindrome(self, x):
		xx = str(x)
		if len(xx) % 2 ==0:
			start = len(xx) / 2 
		else:
			start = (len(xx) + 1)/2			
		for i in range(start,len(xx)):
			if xx[i]  != xx[len(xx)-1-i]:
				return False
		return True

if __name__ == '__main__':

	sol = Solution()
	print sol.isPalindrome(121)
	print sol.isPalindrome(12321)
	print sol.isPalindrome(-1)
	print sol.isPalindrome(1000030001)