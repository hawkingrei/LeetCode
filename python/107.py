class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def levelOrderBottom(self, root: TreeNode) -> List[List[int]]:
        if root == None:
            return []
        result = []
        queue = [(root, 1)]
        while queue:
            (node, deep) = queue.pop(0)
            if len(result) < deep:
                result.insert(0, [])
            result[len(result)-deep].append(node.val)
            if node .left:
                queue.append((node.left, deep+1))
            if node.right:
                queue.append((node.right, deep+1))
        return result
