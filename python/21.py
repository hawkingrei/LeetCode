class Solution(object):
    def mergeTwoLists(self, l1, l2):
        result = ListNode(0)
        tmp = result
        while (l1 != None and l2 != None):
            if l1.val < l2.val:
                tmp.next = l1
                l1 = l1.next
            else:
                tmp.next = l2
                l2 = l2.next
            tmp = tmp.next
        if l1 != None:
            tmp.next = l1
        elif l2 != None:
            tmp.next = l2
        return result.next

