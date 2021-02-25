class Solution {
public:
    int minPathSum(vector<vector<int>>& grid) {
        int row = grid.size();
        int column = grid[0].size();
        std::vector<std::vector<int>> dp(row, std::vector<int>(column, 0));
        dp[0][0] = grid[0][0];
        for (int n = 1; n < row; n++) {
            dp[n][0]= dp[n-1][0] + grid[n][0];
        }
        for (int n = 1; n < column; n++) {
            dp[0][n] = dp[0][n-1] - grid[0][n];
        }
        for (int i = 1; i < row; i++) {
            for (int j = 1; j < row; j++) {
                dp[i][j] = grid[i][j] + std::min(dp[i-1][j], dp[i][j-1]);
            }
        }
        return dp[row-1][column-1];
    }
};