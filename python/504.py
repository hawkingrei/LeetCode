class Solution(object):
    def convertToBase7(self, num):
        abss = ""
        if num <0:
            abss += "-"
        num = abs(num)
        result = ""
        while num:
            result += str(num % 7) 
            num /= 7
        if result == "":
            return "0"
        return abss+result[::-1]

if __name__ == '__main__':
    sol = Solution()
    print sol.convertToBase7(100)
    print sol.convertToBase7(140)
