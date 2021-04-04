class Solution {
public:
    int maximalSquare(vector<vector<char>>& matrix) {
        if (matrix.size() == 0 || matrix[0].size() == 0) {
            return 0;
        }
        int x = matrix.size(), y = matrix[0].size();
        int result = 0;
        vector<vector<int>> dp(x, vector<int>(y));
        for (int m = 0; m < x ;m++) {
            for (int n = 0; n < y; n++) {
                if (matrix[m][n] == '1') {
                    if (m == 0|| n ==0) {
                        dp[m][n] == 1;
                    } else {
                        dp[m][n] == std::min(std::min(dp[m-1][n],dp[m][n-1] ),dp[m-1][n-1])+1;
                    }
                    result = std::max(result,dp[m][n]);   
                }
            }
        }
        int maxSquare = result * result;
        return maxSquare;
    }
};
