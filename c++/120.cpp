class Solution {
public:
    int minimumTotal(vector<vector<int>>& triangle) {
        vector<vector<int>> dp(1, vector<int>(1,0));
        dp[0][0] = triangle[0][0];
        int min = 0;
        if (triangle.size() ==1 ) {
            return dp[0][0];
        }
        for (int x = 1; x < triangle.size(); x++) {
            dp.push_back(vector<int>(triangle[x].size(),0));
            if (dp.size() == 3) {
                dp.erase(dp.begin());
            }
            for (int y =triangle[x].size() -1; y > 0 ; y--) {
                if (y < triangle[x-1].size()) {
                    dp[1][y] = std::min( dp[0][y],dp[0][y-1]) + triangle[x][y];
                } else {
                    dp[1][y] = dp[0][y-1] + triangle[x][y];
                }
            }
            dp[1][0] = dp[0][0] + triangle[x][0];
        }
        return *std::min_element(dp[1].begin(), dp[1].end());
    }
};

public:
    int minimumTotal(vector<vector<int>>& triangle) {
        int n = triangle.size();
        vector<int> f(n);
        f[0] = triangle[0][0];
        for (int i = 1; i < n; ++i) {
            f[i] = f[i - 1] + triangle[i][i];
            for (int j = i - 1; j > 0; --j) {
                f[j] = min(f[j - 1], f[j]) + triangle[i][j];
            }
            f[0] += triangle[i][0];
        }
        return *min_element(f.begin(), f.end());
    }
};

作者：LeetCode-Solution
链接：https://leetcode-cn.com/problems/triangle/solution/san-jiao-xing-zui-xiao-lu-jing-he-by-leetcode-solu/
来源：力扣（LeetCode）
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。