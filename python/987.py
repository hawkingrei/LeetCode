class Solution:
    def mark(self, root, row, col, mark_list):
        mark_list.append((row, col, root.val))
        if root.left:
            self.mark(root.left, row - 1, col + 1, mark_list)
        if root.right:
            self.mark(root.left, row + 1, col + 1, mark_list)

    def verticalTraversal(self, root: TreeNode) -> List[List[int]]:
        mark_list = []
        self.mark(root, 0, 0, mark_list)
        mark_list.sort(key=lambda x: (x[1], x[0], x[2]))

        ans = []
        i = 0
        while i < len(mark_list):
            col_list = [mark_list[i][2]]
            cur_col = mark_list[i][1]
            i += 1
            while i < len(mark_list) and mark_list[i][1] == cur_col:
                col_list.append(mark_list[i][2])
                i += 1
            ans.append(col_list)
        return ans
