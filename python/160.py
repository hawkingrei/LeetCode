class Solution:
    # @param headA: the first list
    # @param headB: the second list
    # @return: a ListNode
    def getIntersectionNode(self, headA, headB):
        # Write your code here
        listA = []
        listB = []
        if headA == None or headB == None:
            return None
        while 1:
            if headA == None:
                break;
            listA.append(headA.val)
            headA = headA.next

        while 1:
            if headB == None:
                break;
            listB.append(headB.val)
            headB = headB.next

        if listA[-1] != listB[-1]:
            return None;
        
        if len(listA)<len(listB):
            minLen = len(listA)
        else:
            minLen = len(listB)

        inster = []
        for i in range(1,minLen+1):
            if listA[-i] != listB[-i]:
                return ListNode(listA[-i+1])
            if i == minLen:
                return ListNode(listA[-i])


#not best
class Solution(object):
    def getIntersectionNode(self, headA, headB):
        """
        :type head1, head1: ListNode
        :rtype: ListNode
        """
        if headA is None or headB is None:
            return None

        pa = headA # 2 pointers
        pb = headB

        while pa is not pb:
            # if either pointer hits the end, switch head and continue the second traversal, 
            # if not hit the end, just move on to next
            pa = headB if pa is None else pa.next
            pb = headA if pb is None else pb.next

        return pa 
