class Solution {
public:
    int maxSubArray(vector<int>& nums) {
        int pre = 0, max_nums = nums[0];
        for (int num: nums) {
            pre = std::max(pre+num,num)
            max_nums = std::max(max_nums,pre)
        }
        return max_nums;
    }
};