/*
 * @lc app=leetcode.cn id=199 lang=cpp
 *
 * [199] 二叉树的右视图
 */

// @lc code=start
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    vector<int> rightSideView(TreeNode* root) {
        std::unordered_map<int,int> rightmostValueAtDepth;
        std::stack<TreeNode*> result;
        std::stack<int> deepth;
        int max_depth = -1;
        result.push(root);
        deepth.push(0);
        while (!result.empty()) {
            TreeNode* node = result.top();
            result.pop();
            int depth = deepth.top();
            
            deepth.pop();
            if (node != nullptr) {
                max_depth = std::max(max_depth, depth);
                if (rightmostValueAtDepth.find(depth) == rightmostValueAtDepth.end()) {
                    rightmostValueAtDepth[depth] = node->val;
                }
                result.push(node->left);
                result.push(node->right);
                
                deepth.push(depth + 1);
                deepth.push(depth + 1);
            }
        }
        std::vector<int> rightView;
        for (int depth = 0; depth <= max_depth; ++depth) {
            rightView.push_back(rightmostValueAtDepth[depth]);
        }
        return rightView;
    }
};
// @lc code=end

