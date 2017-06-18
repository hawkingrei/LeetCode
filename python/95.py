class Solution:
    # @paramn n: An integer
    # @return: A list of root
    def generateTrees(self, n):
        # write your code here
        if n == 0: return []
        return self.dfs(1, n)
        
    def dfs(self, start, end):
        if start > end: return [None]
        res = []
        for rootval in range(start, end+1):
            LeftTree = self.dfs(start, rootval-1)
            RightTree = self.dfs(rootval+1, end)
            for i in LeftTree:
                for j in RightTree:
                    root = TreeNode(rootval)
                    root.left = i
                    root.right = j
                    res.append(root)
        return res
