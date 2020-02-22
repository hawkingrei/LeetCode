class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children


class Solution:
    def levelOrder(self, root: 'Node') -> List[List[int]]:
        if root == None:
            return []
        result = []
        previous_layer = [root]
        while previous_layer:
            current_queue = []
            result.append([])
            for node in previous_layer:
                result[-1].append(node.val)
                current_queue.extend(node.children)
            previous_layer = current_queue
        return result
