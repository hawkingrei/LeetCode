# 1. In place comparison: s[i] == s[n -i -1] ???
# 2. Revese ,a nd compare
# 3. Stack, Queue => Compare every element in stack and queue

class Solution(object):
    def isPalindrome(self, s):
        j = len(s)-1
        i = 0
        while (i < len(s)-1):
            while (i < j and not s[i].isalnum()): 
                i=i+1                
            while (i < j and not s[j].isalnum()): 
                j=j-1
            if (s[i].lower() != s[j].lower()): 
                return False            
            i=i+1
            j=j-1
            if (i >= j):
            	break
        return True

if __name__ == '__main__':
	s = Solution()
	print s.isPalindrome("aba")
	print s.isPalindrome("A man, a plan, a canal: Panama")