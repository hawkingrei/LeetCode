class Solution 
{
public:
    int maxSubArrayLen(vector<int>& nums, int k) 
    {
        int n = nums.size();
        unordered_map<int,int> presum_idx;
        presum_idx[0] = 0;                          
        int presum = 0;
        int res = 0;
        for (int i = 0; i < n; i ++)
        {
            presum += nums[i];
            if (presum_idx.count(presum) == 0)     
                presum_idx[presum] = i + 1;         
            if (presum_idx.count(presum - k))       
                res = max(res, i - presum_idx[presum- k] + 1);
        }
        return res;
    }
};
