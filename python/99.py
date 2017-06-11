class Solution(object):
    def recoverTree(self, root):
        self.lastNode = TreeNode(-1000000000000000000)
        self.firstnode = None
        self.secondnode = None
        self.validate(root)
        if self.firstnode is not None and self.secondnode is not None:
            temp = self.firstnode.val
            self.firstnode.val = self.secondnode.val
            self.secondnode.val = temp;
        
    
    def validate(self, root):
        if root is None:
            return
        self.validate(root.left)
        if self.firstnode is None and self.lastNode.val >= root.val:
            self.firstnode = self.lastNode
        if self.firstnode is not None and self.lastNode.val >= root.val:
            self.secondnode = root
        self.lastNode = root
        self.validate(root.right)
        
        
