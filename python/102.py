class Solution(object):
    def levelOrder(self, root):
        result = []
        def generate(node,level):
            if node == None:
                return
            if len(result) < level:
                result.append([])
            result[level - 1].append(node.val)

            generate(node.left, level + 1)
            generate(node.right, level + 1)
        generate(root,1)
        return result
