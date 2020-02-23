# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def pathSum(self, root: TreeNode, sum: int) -> List[List[int]]:
        result = []

        def bfs(root: TreeNode, total, pa: List[int]):
            if root == None:
                return
            if total == sum and root.right == None and root.left == None:
                pa += [root.val]
                result.append(pa)
            bfs(root.left,  total + root.val, pa + [root.val])
            bfs(root.right, total + root.val, pa + [root.val])
        bfs(root, 0, [])
        return result
