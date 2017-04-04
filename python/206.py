class Solution(object):
    def reverseList(self, head):
        return self._reverse(head)
        """
        :type head: ListNode
        :rtype: ListNode
        """
    def _reverse(self, node, prev=None):
        if not node:
            return prev
        n = node.next
        node.next = prev
        return self._reverse(n, node)