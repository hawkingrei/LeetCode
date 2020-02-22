class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def deepestLeavesSum(self, root: TreeNode) -> int:
        if root.left == None and root.right == None:
            return root.val

        result = []

        def bfs(root: TreeNode, deep: int, result, deepest):
            if root == None:
                return
            if len(result) < deep:
                result.append([])
            if deepest <= deep:
                result[deep - 1].append(root.val)
                deepest = deep

            if root.right:
                bfs(root.right, deep+1, result, deepest)
            if root.left:
                bfs(root.left, deep+1, result, deepest)

        bfs(root, 1, result, 1)
        return sum(result[-1])
