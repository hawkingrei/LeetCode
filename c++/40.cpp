class Solution {
public:
    vector<vector<int>> combinationSum2(vector<int>& candidates, int target) {
        std::vector<std::vector<int>> result;
        std::set<std::vector<int>> res_set;
        std::vector<int> item;
        std::sort(candidates.begin(),candidates.end());
        generate(0,candidates,item,result,res_set,0,target);
        return result;
        

    }
private:
    void generate(int i, std::vector<int>& candidates, 
        std::vector<int> item,
        std::vector<std::vector<int>>& result,
        std::set<std::vector<int>>& res_set,
        int sum,
        int target) {

        if (i >= candidates.size() ||sum > target || candidates[i] > target) {
            return;
        }
        
        sum += candidates[i];
        item.push_back(candidates[i]);
        if (target == sum && res_set.find(item) == res_set.end()){
            result.push_back(item);
            res_set.insert(item);
        }
        generate(i+1, candidates,item,result,res_set,sum,target);
        sum = sum - candidates[i];
        item.pop_back();        
        generate(i+1, candidates,item,result,res_set,sum,target);
    }
};