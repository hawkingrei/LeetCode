dx = [1,-1,0,0]
dy = [0,0,1,-1]

class Solution(object):
    def totalNQueens(self, n):
    	if n < 1 : return []
    	self.result = []
    	self.cols = set()
    	self.pie = set()
    	self.na = set()
    	self.count = 0
    	self.DFS(n,0,[])
    	return self.count

    def DFS(self,n,row,cur_state):
    	if row >= n:
    		self.count += 1
    		return

    	for col in range(n):
    		if col in self.cols or row + col in self.pie or row -col in self.na:
    			continue
    		self.cols.add(col)
    		self.pie.add(row+col)
    		self.na.add(row-col)

    		self.DFS(n,row+1,cur_state +[col])
    		self.cols.remove(col)
    		self.pie.remove(row+col)
    		self.na.remove(row-col)

    def _generate_result(self,n):
    	board = []
    	for res in self.result:
    		for i in res:
    			board.append("." * i + "Q" + '.'*(n-i-1))

    	return [board[i:i+n] for i in range(0,len(board),n)]

if __name__ == '__main__':
	sol = Solution()
	print sol.solveNQueens(3)
	print sol.solveNQueens(4)
	print sol.solveNQueens(5)
