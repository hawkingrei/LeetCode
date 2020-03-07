class Solution:
    def lowestCommonAncestor(self, root: 'TreeNode', p: 'TreeNode', q: 'TreeNode') -> 'TreeNode':
        parentVal = root.val
        pVal = p.val
        qVal = q.val
        if (pVal > parentVal and qVal > parentVal):
            return self.lowestCommonAncestor(root.right, p, q)
        elif (pVal < parentVal and qVal < parentVal):
            return self.lowestCommonAncestor(root.left, p, q)
        else:
            return root
