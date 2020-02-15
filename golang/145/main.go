package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func postorderTraversal(root *TreeNode) []int {
	var (
		li    []int
		stack []*TreeNode
	)
	for {
		if len(stack) == 0 && root == nil {
			break
		}
		if root != nil {
			li = append(li, root.Val)
			stack = append(stack, root)
			root = root.Right
		} else {
			root = stack[len(stack)-1].Left
			stack = stack[:len(stack)-1]
		}
	}
	reverse(li)
	return li
}

func reverse(li []int) {
	length := len(li)
	for i := 0; i < length/2; i++ {
		li[i], li[length-i-1] = li[length-i-1], li[i]
	}
}
