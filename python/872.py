class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def leave_val(self, root: TreeNode) -> List[int]:
        queue = [root]
        result = []
        while queue:
            node = queue.pop()
            if node.right == None and node.left == None:
                result.append(node.val)
                continue
            if node.right:
                queue.append(node.right)
            if node.left:
                queue.append(node.left)
        return result

    def leafSimilar(self, root1: TreeNode, root2: TreeNode) -> bool:
        return self.leave_val(root1) == self.leave_val(root2)
