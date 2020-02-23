# class TreeNode:
import sys


def __init__(self, x):
    self.val = x
    self.left = None
    self.right = None


class Solution:
    def minDepth(self, root: TreeNode) -> int:
        if root == None:
            return 0

        def dfs(root: TreeNode, deep):
            if root.left == None and root.right == None:
                if deep < smalldeep:
                    smalldeep = deep

            if root.left:
                def(root.left, deep+1, smalldeep)
            if root.right:
                def(root.right, deep+1, smalldeep)

        smalldeep = sys.maxsize
        dfs(root, deep, 1, smalldeep)
        return smalldeep
