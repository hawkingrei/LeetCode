package reverse

type ListNode struct {
	Val  int
	Next *ListNode
}

func reverse(start *ListNode, end *ListNode) *ListNode {
	head := end
	for start != end {
		next := start.Next
		start.Next = head
		head = start
		start = next
	}
	return head
}

func reverseKGroup(head *ListNode, k int) *ListNode {
    node :=	head
	for i := 0; i < k; i=i+1 {
		if node == nil {
			return head
		}
		node = node.Next
	}
	newHead := reverse(head, node)
	head.Next = reverseKGroup(node, k)
	return newHead
}