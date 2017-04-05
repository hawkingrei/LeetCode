class Solution(object):
    def largestValues(self, root):
        result = []
        def generate(node,level):
            if (node == None):
                return
            if len(result)<level:
                result.append(node.val)
            else:
                if result[level-1]<node.val:
                    result[level-1]=node.val
            generate(node.left,level+1)
            generate(node.right,level+1)
        generate(root,1)
        return result
