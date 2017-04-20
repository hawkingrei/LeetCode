class Solution(object):
    def searchInsert(self, nums, target):
    	if len(nums) == 0:
    		return 0
    	if target in nums:
    		return nums.index(target)
    	else:
    		nums.append(target)
    		nums.sort()
    		return nums.index(target)
    		
if __name__ == '__main__':
	sol = Solution()
	print sol.searchInsert([1,3,5,6], 5)
	print sol.searchInsert([1,3,5,6], 2)
	print sol.searchInsert([1,3,5,6], 7)
	print sol.searchInsert([1,3,5,6], 0)