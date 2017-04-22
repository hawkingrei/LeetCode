class Solution(object):
    def fizzBuzz(self, n):
        result = [ "" for _ in range(1,n+1)]
        for x in range(n):
            if (x+1) % 3 == 0:
                result[x] = "Fizz"
            if (x+1) % 5 == 0:
                result[x] += "Buzz"
            if result[x] == "":
                result[x] = str(x+1)
        return result

if __name__ == '__main__':
    sol = Solution()
    print sol.fizzBuzz(15)