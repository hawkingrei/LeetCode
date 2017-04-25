class Solution(object):
    def convertToTitle(self, n):
    	result = ""
    	while n:
    		l = n % 26
    		if l == 0:
    			result += "Z"
    			n = n -  1
    		else:
    			result += chr(64+l)
    		n /= 26
    	return result[::-1]

if __name__ == '__main__':
	sol = Solution()
	print sol.convertToTitle(26)
	print sol.convertToTitle(27)
	print sol.convertToTitle(28)
	print sol.convertToTitle(29)
	print sol.convertToTitle(30)
	print sol.convertToTitle(52)