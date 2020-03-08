# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def maxPathSum(self, root: TreeNode) -> int:
        global maxsum
        maxsum = -99999999999999

        def bfs(root: TreeNode):
            global maxsum
            if not root:
                return 0
            lv = max(bfs(root.left), 0)
            rv = max(bfs(root.right), 0)
            sm = root.val + lv + rv
            if sm > maxsum:
                maxsum = sm
            return root.val + max(lv, rv)

        bfs(root)
        return maxsum
