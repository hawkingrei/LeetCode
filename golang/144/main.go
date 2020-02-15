package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func preorderTraversal(root *TreeNode) []int {

}

package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func preorderTraversal(root *TreeNode) []int {
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
	*result = append(*result,root.Val)
	gofunc(root.Left, result)
	gofunc(root.Right, result)
}
