class Solution(object):
    def minPathSum(self, grid):
        m = len(grid)
        n = len(grid[0])
        for x in range(m):
            for y in range(n):
                if x == 0 and y > 0:
                    grid[x][y] += grid[x][y-1]
                elif y == 0 and x > 0:
                    grid[x][y] += grid[x-1][y]
                elif x > 0 and y > 0:
                    grid[x][y] += min(grid[x-1][y],grid[x][y-1])
        return grid[m-1][n-1]
