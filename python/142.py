#slow using dict
class Solution(object):
    def detectCycle(self, head):
        p = head
        ht = dict()
        while p:
            if id(p) in ht:
                return p
            else:
                ht[id(p)] = True
            p = p.next
        return None

#quick
#http://blog.csdn.net/kenden23/article/details/13871699
class Solution(object):
    def detectCycle(self, head):
        fast = slow = head
        while fast and fast.next :
            fast = fast.next.next
            slow = slow.next
            if fast == slow:
                break
        else:
            return None
        while head != slow:
            slow = slow.next
            head = head.next
        return slow 

        

