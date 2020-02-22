class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children


class Solution:
    def preorder(self, root: 'Node') -> List[int]:
        if root == None:
            return []
        queue, result = [root], []
        while queue:
            node = queue.pop()
            result.append(node.val)
            queue.extend(node.children[::-1])
        return result
