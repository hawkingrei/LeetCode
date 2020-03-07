# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def binaryTreePaths(self, root: TreeNode,) -> List[str]:
        result = []

        def bfs(root: TreeNode, path):
            if not root:
                return
            p = path + str(root.val)
            if not root.left and not root.right:
                result.append(p)
            else:
                p = p + "->"
                bfs(root.left, p)
                bfs(root.right, p)

        bfs(root, "")
        return result
