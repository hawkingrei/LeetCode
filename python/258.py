class Solution(object):
	def addDigits(self, num):
		while num >=10:
			tmpnum = num
			total = 0
			while tmpnum != 0:
				total += tmpnum % 10
				tmpnum = tmpnum / 10
			num = total
		return num
        

if __name__ == '__main__':
	sol = Solution()
	print sol.addDigits(10)
	print sol.addDigits(234)
	print sol.addDigits(48)
