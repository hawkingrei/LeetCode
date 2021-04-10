/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
public:
    std::unordered_map<int, TreeNode*> fa;
    std::unordered_map<int, bool> visited;
    void bfs(TreeNode* root) {
            if (root->right) {
                fa[root->right->val] = root;
                bfs(root->right);
            }
            if (root->left) {
                fa[root->left->val] = root;
                bfs(root->left);
            }
    }
    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
        fa[root->val] = nullptr;
        bfs(root);
        while (p) {
            visited[p->val] = true;
            p = fa[p->val];
        }
        while (q) {
            if (visited[q->val]) {
                return q;
            }
            q = fa[q->val];
        }
        return nullptr;
    }
};