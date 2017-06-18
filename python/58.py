class Solution(object):
    def lengthOfLastWord(self, s):
        """
        :type s: str
        :rtype: int
        """
        if len(s) == 0 or s== ' ': return 0
        ss = s.split(' ')
        for index in range(len(ss)-1,-1,-1):
            slong = len(ss[index])
            if  slong == 0:
                continue
            else:
                return slong
        return 0
