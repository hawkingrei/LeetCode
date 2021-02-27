class Solution {
public:
    void reserve(vector<int>& nums, int start, int end) {
        while (start < end) {
            std::swap(nums[start], nums[end]);
            start +=1;
            end -=1;
        }
    }
    void rotate(vector<int>& nums, int k) {
        k %= nums.size();
        reserve(nums,0,nums.size()-1);
        reserve(nums,0,k-1);
        reserve(nums,k,nums.size()-1);
    }
};