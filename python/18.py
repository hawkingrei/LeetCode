class Solution(object):
    def fourSum(self, nums, target):
        nums.sort()
        res = []
        length = len(nums) - 1
        for left in range(length-2):
            if left and nums[left] == nums[left - 1]:
                continue
            for right in range(length,left+2    ,-1):
                if right < length and nums[right] == nums[right + 1]:
                    continue
                tsum = target - (nums[right] + nums[left])
                midl ,midr = left + 1, right -1
                while midl < midr:
                    if nums[midl] + nums[midr] == tsum:
                        res.append([nums[left], nums[midl], nums[midr],nums[right]])
                        midl += 1
                        midr -= 1
                        while nums[midl] == nums[midl - 1]  and midl < midr  :
                            midl += 1
                        while nums[midr] == nums[midr + 1]  and midl < midr  : 
                            midr -= 1
                    else:
                        if nums[midl] + nums[midr] > tsum:
                            midr -= 1
                        else:
                            midl += 1
        return res 

if __name__ == '__main__':
	sol = Solution()
	print sol.fourSum([9,6,4,9,8,-2,4,0,6,-2,-8,6,-3,9,10],25)
