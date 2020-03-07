# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def reserve(self, root: TreeNode, mp, sm, target_sum):
        if not root:
            return
        current_sum = sm + root.val
        if current_sum-target_sum in mp:
            self.count += mp[current_sum-target_sum]
        mp[current_sum] = mp.get(current_sum, 0) + 1
        self.reserve(root.left, mp, current_sum, target_sum)
        self.reserve(root.right, mp, current_sum, target_sum)
        mp[current_sum] -= 1

    def pathSum(self, root: TreeNode, sum: int) -> int:
        self.count = 0
        self.reserve(root, {0: 1}, 0, sum)
        return self.count
