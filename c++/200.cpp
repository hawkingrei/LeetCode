/*
 * @lc app=leetcode.cn id=200 lang=cpp
 *
 * [200] 岛屿数量
 */

// @lc code=start
class Solution {
public:
    int numIslands(vector<vector<char>>& grid) {
        std::vector<std::vector<int>> visited = std::vector(grid.size(),std::vector(grid[0].size(),0));
        int total = 0;
        for (int i = 0; i< grid.size(); i++) {
            for (int j = 0; j< grid[0].size(); j++) {
               if (visited[i][j] == 1) {
                   continue;
               }
               if (grid[i][j] == '1') {
                   visitedIsland(i,j,grid,visited);
                   total++;
               }
            }
        }
        return total;
    }
private:
    void visitedIsland(int x,int y,vector<vector<char>>& grid, vector<vector<int>>& visited) {
        if (x > grid.size()-1 || y > grid[0].size()-1) {
            return;
        }
        if (visited[x][y] == 1) {
            return;
        }

        if (grid[x][y] == '1') {
            visited[x][y] = 1;
            visitedIsland(x+1,y,grid,visited);
            visitedIsland(x-1,y,grid,visited);
            visitedIsland(x,y+1,grid,visited);
            visitedIsland(x,y-1,grid,visited);
        } else {
            return;
        }
    }
};
// @lc code=end