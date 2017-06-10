class Solution(object):
    def isValidBST(self, root):
        self.lastVal = None
        self.isBST = True
        self.validate(root)
        return self.isBST

    def validate(self, root):
        if root is None:
            return
        self.validate(root.left)
        if self.lastVal is not None and self.lastVal >= root.val:
            self.isBST = False
            return
        self.lastVal = root.val
        self.validate(root.right)
