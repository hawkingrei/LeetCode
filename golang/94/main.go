package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func inorderTraversal(root *TreeNode) []int {
	if root == nil {
		return []int{}
	}
	var result []int
	gofunc(root, &result)
	return result
}

func gofunc(root *TreeNode, result *[]int) {
	if root == nil {
		return
	}
	gofunc(root.Left, result)
	*result = append(*result,root.Val)
	gofunc(root.Right, result)
}
