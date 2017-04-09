class Solution(object):
    def diameterOfBinaryTree(self, root):
        self.maxmax = 0
        def generate(result,node):
            if node == None:
                return 0
            
            maxleft = generate(result,node.left)
            maxright = generate(result,node.right)
            self.maxmax = max(self.maxmax,maxleft+maxright)
            return max(maxright,maxleft) +1
        if not root:
            return 0
        generate(0,root)
        return self.maxmax
