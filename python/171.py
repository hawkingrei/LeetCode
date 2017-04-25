class Solution(object):
    def titleToNumber(self, s):
        result = 0
        l = len(s)
        for index in range(l):
            print (ord(s[index])-64)*26**(l-1-index)
            result += (ord(s[index])-64)*26**(l-1-index)
        return result

if __name__ == '__main__':
    sol = Solution()
    print sol.titleToNumber('AA')