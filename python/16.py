class Solution(object):
    def threeSumClosest(self, numbers, target):
        numbers.sort()
        ans = None
        for i in range(len(numbers)):
            l, r = i + 1, len(numbers) - 1
            while (l < r):                    
                sum = numbers[l] + numbers[r] + numbers[i]
                if ans is None or abs(sum- target) < abs(ans - target):
                    ans = sum
                if sum <= target:
                    l = l + 1
                else:
                    r = r - 1
        return ans

if __name__ == '__main__':
	sol = Solution()
	print sol.threeSumClosest([-1, 2, 1, -4],1)