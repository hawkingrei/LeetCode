# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def isSame(self, s: TreeNode, t: TreeNode) -> bool:
        if s.val != t.val:
            return False
        right, left = False, False
        if s.left == None and t.left == None:
            left = True
        elif (s.left != None and t.left != None):
            left = self.isSame(s.left, t.left)

        if s.right == None and t.right == None:
            right = True
        elif (s.right != None and t.right != None):
            right = self.isSame(s.right, t.right)
        return right and left

    def isSubtree(self, s: TreeNode, t: TreeNode) -> bool:
        if self.isSame(s, t):
            return True

        right, left = False, False
        if s.right:
            right = self.isSubtree(s.right, t)
            if right:
                return True
        if s.left:
            left = self.isSubtree(s.left, t)
            if left:
                return True
        return False
