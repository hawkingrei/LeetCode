class Solution(object):
    def climbStairs(self, n):
        def generate(way,ways,n):
            if way < n:
                a = 0
                b = 0
                way +=  1
                a += generate(way,ways,n)
                way = way -1 + 2
                b += generate(way,ways,n)
                way -= 2
                return a+b
            if way == n:
                return 1
            if way > n:
                return 0
        return generate(0,0,n)    #  time limit

class Solution(object):
    def climbStairs(self, n):
        ways = [1,1];  
        for i in range(1,n):  
            temp = ways[1]
            ways[1] += ways[0]  
            ways[0] = temp
        return ways[1]

if __name__ == '__main__':
    sol =Solution()
    print sol.climbStairs(2)
    print sol.climbStairs(3)
    print sol.climbStairs(4)
    print sol.climbStairs(5)
    print sol.climbStairs(6)