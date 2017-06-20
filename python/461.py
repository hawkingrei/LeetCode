class Solution(object):
    def hammingDistance(self, x, y):
        i = x ^ y
        count=0
        while (i != 0):
            count +=1
            i = (i-1)& i
        return count
