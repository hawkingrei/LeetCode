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



class Solution(object):
    def mergeTwoLists(self, l1, l2):
        if not l1 or not l2 : return l1 or l2
        if l1.val < l2.val:
            l1.next = self.mergeTwoLists(l1.next,l2)
        else:
            l2.next = self.mergeTwoLists(l1,l2.next)
        return l1 if l1.val < l2.val else l2 