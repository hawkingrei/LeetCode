class Solution {
public:
    vector<vector<int>> combinationSum(vector<int>& candidates, int target) {
        vector<vector<int>> result;
        vector<int> tmp;
        bfs(candidates, target,result, tmp,0);
        return result;
    }
private:
    void bfs(vector<int>& candidates, int target, vector<vector<int>>& result,vector<int>& tmp, int idx) {
        if (idx == candidates.size()) {
            return; 
        }
        if (target == 0) {
            result.push_back(tmp);
            return;
        }
        //跳过
        bfs(candidates,target, result,tmp,idx +1);
        if (target - candidates[idx] >= 0) {
            tmp.push_back(candidates[idx]);
            bfs(candidates,target- candidates[idx], result,tmp,idx);
            tmp.pop_back();
        }
    }
};