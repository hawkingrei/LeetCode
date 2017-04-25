class Solution(object):
    def sumOfLeftLeaves(self, root):
        if root == None:
            return 0
        def dfs(node):
            if node == None: return 0
            if node.left != None and node.left.left == None and node.left.right == None:
                return dfs(node.right)+node.left.val
            return dfs(node.right)+dfs(node.left)
        return dfs(root)
