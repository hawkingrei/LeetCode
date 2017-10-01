class NumArray(object):
    def __init__(self, nums):
        """
        initialize your data structure here.
        :type nums: List[int]
        """
        self.sums = [0] * (len(nums) + 1)
        self.nums = nums
        self.n = len(nums)
        for i in xrange(len(nums)):
            self.add(i + 1,nums[i])

    def add(self,x,val):
        while x <= self.n:
            self.sums[x] += val
            x += self.lowbit(x)

    def lowbit(self,x):
        return x & -x

    def sum(self,x):
        res = 0
        while x > 0:
            res += self.sums[x]
            x -= self.lowbit(x)
        return res

    def update(self, i, val):
        """
        :type i: int
        :type val: int
        :rtype: int
        """
        self.add(i + 1, val - self.nums[i])
        self.nums[i] = val

    def sumRange(self, i, j):
        """
        sum of elements nums[i..j], inclusive.
        :type i: int
        :type j: int
        :rtype: int
        """
        if not self.nums:
            return  0
        return self.sum(j+1) - self.sum(i)
