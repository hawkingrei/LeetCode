class Solution(object):
    def deleteDuplicates(self, head):
        if head == None:
            return head
        tmp = head
        while head != None and head.next != None:
            if head.val == head.next.val:
                head.val ,head.next = head.next.val, head.next.next
            else:
                head = head.next
        return tmp
