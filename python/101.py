class Solution(object):
    def isSymmetric(self, root):
        if root == None: return True
        def generate(rnode,lnode):
            if rnode == None and lnode == None: return True
            if (rnode != None and lnode == None) or (rnode == None and lnode != None):
                return False
            if (rnode == lnode):
                return generate(rnode.right,rnode.left)
            if (rnode.val != lnode.val):
                return False
            return generate(rnode.right,lnode.left) and generate(rnode.left,lnode.right)
        return generate(root,root)
