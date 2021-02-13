#include <stdio.h>
#include <map>
#include <string>

class Solution {
public:
    int longestPalindrome(string s) {
        int char_map[1010] = {0};
        int max_length = 0;
        int flag = 0;
        for (int i = 0; i< s.length(); i++) {
            char_map[s[i]]++;
        }
        for (int i = 0; i < 1010; i++) {
            if (char_map[i] % 2 == 0) {
                max_length += char_map[i];
            } else {
                max_length += char_map[i] - 1;
                flag = 1;
            }
        }
        return max_length + flag;
    }
};