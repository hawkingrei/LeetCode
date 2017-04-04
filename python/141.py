class Solution(object):
    def hasCycle(self, head):
        onespeed = head
        twospeed = head
        try:
            while True:
                if onespeed.next == twospeed.next.next:
                    return True
                onespeed , twospeed= onespeed.next,twospeed.next.next
        except:
            return False