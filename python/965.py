class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def isUnivalTree(self, root: TreeNode) -> bool:
        val = root.val
        queue = [root]
        result = True
        while queue and result:
            node = queue.pop()
            if node.val != val:
                result = False
                break
            if node.left:
                queue.append(node.left)
            if node.right:
                queue.append(node.right)
        return result
