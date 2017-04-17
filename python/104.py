class Solution(object):
    def maxDepth(self, root):
        if root == None:
            return 0
        return max(self.maxDepth(root.left),self.maxDepth(root.right)) + 1
