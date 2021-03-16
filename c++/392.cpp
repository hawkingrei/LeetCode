class Solution {
public:
    bool isSubsequence(string s, string t) {
        if (s == t) {
            return true;
        }
        if (s == "") {
            return true;
        }
        std::vector<int> dp(t.size(),0);
        dp[0] = (s[0] == t[0]) ? 1 : 0;
        for (int i = 1; i < t.size();i++) {
            if (t[i] == s[dp[i-1]]) {
                dp[i] = dp[i-1] +1;
            } else {
                dp[i] = dp[i-1];
            }
        }
        return dp[t.size()-1] == s.size();
    }
};