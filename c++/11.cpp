class Solution {
public:
    int maxArea(vector<int>& height) {
        int begin = 0;
        int end = height.size()-1;
        int max = 0;
        while (begin< end) {
             max = std::max(max,std::min(height[begin],height[end])*(end -begin));
             if (height[begin] > height[end]) {
                 end = end -1;
             } else {
                 ++begin;
             }
        }
        return max;
    }
};
