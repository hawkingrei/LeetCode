class Solution(object):
    def search(self,nums,result,index):
        if index == len(nums):
            self.results.append(result)
            return
        self.search(nums,result +[nums[index]],index+1)
        self.search(nums,result,index+1)
        
    def subsets(self, nums):
        self.results = []
        self.search(sorted(nums),[],0)
        return self.results
