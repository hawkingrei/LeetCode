class Solution(object):
    def isValid(self, s):
    	tmp    = []
    	intmp  = ["[","{","("]	
    	outtmp = ["]","}",")"]
    	for word in s:
    		if word in intmp:
    			tmp.append(word)
    			continue
    		if word in outtmp:
    			if len(tmp) ==0:
    				return False
    			if tmp[len(tmp)-1] == intmp[outtmp.index(word)]:
    				tmp.pop()
    			else:
    				break
    	if len(tmp) == 0:
    		return True
    	return False

if __name__ == '__main__':
	s = Solution()
	print s.isValid("()")