class Solution(object):
    def countAndSay(self, n):
        start = "1"
        tmp = ""
        for x in range(n-1):
            index = 0
            while True:
            	
                if index > len(start) - 1:
                    break
                if index + 1 < len(start) and start[index] == start[index+1]:
                    sum = 1
                    index += 1
                    while True:
                        sum +=1
                        if index + 1 >= len(start) or start[index] != start[index + 1]:
                            break
                        index = index + 1    
                    tmp += str(sum) + start[index]
                else:
                    tmp += "1" + start[index]
                index += 1
            start = tmp
            tmp = ""
            print start
        return start

if __name__ == '__main__':
	sol = Solution()
	print sol.countAndSay(1)