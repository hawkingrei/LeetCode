class Solution:
    def longestUnivaluePath(self, root: TreeNode) -> int:
        global maxnum
        maxnum = 0

        def dfs(root: TreeNode) -> int:
            global maxnum
            if not root:
                return 0
            ln = dfs(root.left)
            rn = dfs(root.right)
            left_arrow = right_arrow = 0
            if root.left and root.left.val == root.val:
                left_arrow = ln + 1
            if root.right and root.right.val == root.val:
                right_arrow = rn + 1
            maxnum = max(maxnum, right_arrow+left_arrow)
            return max(right_arrow, left_arrow)

        dfs(root)
        return maxnum
