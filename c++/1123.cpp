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
    TreeNode* lcaDeepestLeaves(TreeNode* root) {
        int height;
        return find(root, height);
    }

    TreeNode* find(TreeNode* root, int& height) {
        if (!root) {
            height = -1;
            return NULL;
        }
        int leftHeight;
        int rightHeight;
        auto left = find(root->left, leftHeight);
        auto right = find(root->right, rightHeight);
        height = max(leftHeight, rightHeight) + 1;
        if (leftHeight == rightHeight) return root;
        if (leftHeight > rightHeight) return left;
        return right;
    }
};
