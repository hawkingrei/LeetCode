class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children


class Solution:
    def postorder(self, root: 'Node') -> List[int]:
        if root == None:
            return []

        queue = [root]
        result = []

        while queue:
            node = queue.pop()
            result.append(node.val)
            queue.extend(node.children)
        return result[::-1]
