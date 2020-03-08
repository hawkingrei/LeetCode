# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def findFrequentTreeSum(self, root: TreeNode) -> List[int]:
        mp = {}

        def bfs(root: TreeNode) -> int:
            if not root:
                return 0
            result = root.val + bfs(root.left) + bfs(root.right)
            if result in mp:
                mp[result] += 1
            else:
                mp[result] = 1
            return result
        bfs(root)
        result = []
        maxnum = 0
        for m in mp:
            if mp[m] == maxnum:
                result.append(m)
            elif mp[m] > maxnum:
                result = []
                result.append(m)
                maxnum = mp[m]
        return result
