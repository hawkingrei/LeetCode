class Solution(object):
    def reverse(self, x):
    	if x == 0:
    		return 0
    	negative = 1
    	if x < 0:
    		negative, x = -1, -x
    	reverse = 0
    	while x > 0:
    		reverse = reverse * 10 + x % 10
    		x = x / 10

    	reverse = reverse * negative

    	if reverse < -(1 << 31) or reverse > (1 << 31) - 1:
            return 0
        return reverse

if __name__ == '__main__':
	sol = Solution() 
	print sol.reverse(100)
	print sol.reverse(145)
	print sol.reverse(-100)
	print sol.reverse(-145)