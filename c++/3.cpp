class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        int begin = 0;
        int result = 0;
        std::string word = "";
        int char_map[128] = {0};
        for (int i = 0; i < s.length(); i++) {
            char_map[s[i]]++;
            if (char_map[s[i]] == 1) {
                word += s[i];
            }
            if (result < word.length()) {
                result = word.length();
            } else {
                while (begin < i && char_map[s[i]] > 1) {
                    char_map[s[begin]]--;
                    begin++;
                }
                word = "";
                for (int j = begin; j <= i;j++) {
                    word += s[j];
                }
            }
        }
        return result;
    }
};