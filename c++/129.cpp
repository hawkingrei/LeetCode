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
    void dfs(TreeNode* root,vector<int> line_num,int* cnt) {
        line_num.push_back(root->val);
        if (root->right) {
            dfs(root->right,line_num,cnt);
        }
        if (root->left) {
            dfs(root->left,line_num,cnt);
        }
        if (root->right == nullptr && root->left == nullptr) {
            int total = 0;
            for (auto n : line_num) {
               total = total*10 + n;
            }
            *cnt = *cnt + total;
        }

    }

    int sumNumbers(TreeNode* root) {
        int total = 0;
        dfs(root, {}, &total);
        return total;
    }
};