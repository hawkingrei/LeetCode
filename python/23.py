from Queue import PriorityQueue

class Solution(object):
    def mergeKLists(self, lists):
    	vhead = ListNode(None)
    	pre = vhead
    	priority_queue = PriorityQueue()

    	for head in lists:
    		if head: priority_queue.put((head.val,head))

    	while priority_queue.qsize() > 0:
    		pre.next = priority_queue.get()[1]
    		pre = pre.next;
    		next_node = pre.next
    		if next_node: priority_queue.put((next_node.val,next_node))
    	return vhead.next