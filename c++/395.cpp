#include <map>
// @lc code=start
class Solution {
public:
    int dfs(string s,int start, int end, int k) {
        vector<int> cnt(26, 0);
        int result = 0;
        for (int i = start; i <= end ; i++) {
            cnt[s[i]-'a']++;
        }
        char brk = 0;
        for (int i = 0; i< 26; i++) {
            if (cnt[i] > 0 && cnt[i] < k) {
                brk = i + 'a';
                break;
            }
        } 
        if (brk == 0) {
            return end - start +1;
        }
        while (start <= end) {
            while (s[start] == brk && start <= end) {
                start++;
            }
            if (start > end) {
                break;
            }
            int l = start;
            while (s[start] != brk && start <= end) {
                start++;
            }
            int length = dfs(s,l,start-1,k);
            result = std::max(result,length);
        }
        return result;
    }
    int longestSubstring(string s, int k) {
        int size = s.size();
        return dfs(s,0,size - 1,k);
    }
};