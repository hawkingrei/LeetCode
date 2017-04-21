class Solution(object):
    def deleteNode(self, node):
        node.val ,node.next = node.next.val ,node.next.next
