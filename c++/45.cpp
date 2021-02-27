class Solution {
public:
    int jump(vector<int>& nums) {
        int end = 0;
        int max = 0;
        int step = 0;
        for (int i = 0; i< nums.size() -1; i++) {
            if (max >= i) {
                max = std::max(max, i + nums[i]);
                if (i == end){
                    end = max;
                    step++;
                }
            }
        }
        return step;
    }
};