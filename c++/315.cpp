class Solution {
public:
    vector<int> countSmaller(vector<int>& nums) {
        std::vector<std::pair<int,int>> vec;
        std::vector<int> count;
        for (int i = 0; i < nums.size(); i++) {
            vec.push_back(std::make_pair(nums[i],i));
            count.push_back(0);
        }
        merge_sort(vec, count);
        return count;
    }
private:
    void merge_sort_two_vec(
        std::vector<std::pair<int,int>> &sub_vec1,
        std::vector<std::pair<int,int>> &sub_vec2,
        std::vector<std::pair<int,int>> &vec,
        std::vector<int> &count
    ) {
        int i = 0;
        int j = 0;
        while (i < sub_vec1.size() && j < sub_vec2.size()) {
            if (sub_vec1[i].first <= sub_vec2[j].first) {
                count[sub_vec1[i].second] += j;
                vec.push_back(sub_vec1[i]);
                i++;
            } else {
                vec.push_back(sub_vec2[j]);
                j++;
            }
        }
        for (;i< sub_vec1.size();i++) {
            count[sub_vec[i].second] += j;
            vec.push_back(sub_vec1[i]);  
        }
        for (;i< sub_vec2.size();i++) {
            vec.push_back(sub_vec2[i]);
        }
    }

    void merge_sort(
        std::vector<std::pair<int,int>> &vec,
        std::vector<int> &count
    ) {
        if (vec.size()<2) {
            return;
        }
        int mid = vec.size() / 2;
        std::vector<std::pair<int,int>> sub_vec1;
        std::vector<std::pair<int,int>> sub_vec2;
        for (int i = 0; i < mid; i++) {
            sub_vec1.push_back(vec[i]);
        }
        for (int i = mid; i < vec.size();i++) {
            sub_vec2.push_back(vec[i]);
        }
        merge_sort(sub_vec1,count);
        merge_sort(sub_vec2,count);
        vec.clear();
        merge_sort_two_vec(sub_vec1,sub_vec2,vec,count);
    }
};