class Solution(object):
    def wordBreak(self, s, wordDict):
        if len(wordDict) == 0:
            return False
        if len(s)>sum([len(n) for n in wordDict]):
            tmpbomb = True
            for n in wordDict:
                if len(n) == len(s) - sum([len(n) for n in wordDict]):
                    tmpbomb = False
            if tmpbomb:
                return False
        dp = [[ False for _ in s ] for _ in wordDict]
        for wordn in range(len(wordDict)):
            tdp = [[ False for _ in s ] for _ in range(len(wordDict[wordn])+1)  ]
            for n in range(len(s)):
                tdp[0][n] = True
            for i in range(1,len(wordDict[wordn])+1):
                for j in range(len(s)):
                    if wordDict[wordn][i-1] == s[j]:
                        if j != 0 and i != 1:
                            tdp[i][j] = True and tdp[i-1][j-1]
                            if i - 1 == 1 and j - 1 == 0:   
                                tdp[i][j-1] = True 
                            tmp = j
                            while tdp[i][j]:
                                if j < 1:
                                    break
                                if tdp[i-1][j-1]:
                                    tdp[i][j-1] = True
                                    j = j -1
                                else:
                                    break
                            j = tmp
                        else:
                            if i != 1:
                                tdp[i][j] = True and tdp[i][j-1]
                            else:
                                tdp[i][j] = True                    
                    else:
                        tdp[i][j] = False
            for i in range(len(s)):         
                dp[wordn][i] = tdp[len(wordDict[wordn])][i]
        judge = [True for _ in s]
        for i in range(len(wordDict)):
            if  judge == dp[i]:
                return True
        for i in range(len(s)):
            result = dp[0][i]
            for j in range(len(wordDict)):
                result = result or dp[j][i]
            if not result:
                return False
        return True

if __name__ == '__main__':
	sol = Solution()

	print sol.wordBreak("abcde",["ab","cde"])
	print sol.wordBreak("ABCDE",["ABC","BCDE"])
	print sol.wordBreak("ABCDE",["AC","BCDE"])
	print sol.wordBreak("ABCDE",["A","BCDE"])
	print sol.wordBreak("bb",["a","b","bbb","bbbb"])
	print sol.wordBreak("aaaaaaa",["aaaa","aa"])
	print sol.wordBreak("leetcode",["leet","code"])
	print sol.wordBreak("aaaaaaaa",["aaaa","aa","a"])
	print sol.wordBreak("cbca",["bc","ca"])
