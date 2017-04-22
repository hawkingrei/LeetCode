class Solution(object):
    def removeElements(self, head, val):
        result = ListNode(-1)
        result.next = head
        tmp = result
        while tmp != None and tmp.next != None:
            if tmp.next.val == val:
                tmp.next = tmp.next.next
            else:
                tmp = tmp.next
        return result.next
