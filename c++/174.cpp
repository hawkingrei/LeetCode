class Solution {
public:
    int calculateMinimumHP(std::vector<std::vector<int> >& dungeon) {
    	if (dungeon.size() == 0){
	    	return 0;
	    }
    	std::vector<std::vector<int> > 
			dp(dungeon.size(), std::vector<int>(dungeon[0].size(), 0));
    	int row = dungeon.size();
    	int column = dungeon[0].size();
    	dp[row-1][column-1] = std::max(1, 1-dungeon[row-1][column-1]);
    	for (int i = column-2; i>=0; i--){
	    	dp[row-1][i] = std::max(1,
									dp[row-1][i+1] - dungeon[row-1][i]);
	    }
	    for (int i = row-2; i>=0; i--){
	    	dp[i][column-1] = std::max(1,
								dp[i+1][column-1] - dungeon[i][column-1]);
	    }
	    for (int i = row-2; i>=0; i--){
    		for (int j = column-2; j>=0; j--){
    			int dp_min = std::min(dp[i+1][j], dp[i][j+1]);
		    	dp[i][j] = std::max(1, dp_min - dungeon[i][j]);
		    }
    	}
	    return dp[0][0];
    }
};
