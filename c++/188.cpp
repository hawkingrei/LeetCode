class Solution {
public:
    void reserse(vector<char>& s, int start,int end) {
        while (start< end) {
            swap(s[start],s[end]);
            start +=1;
            end -=1;
        }
    }
    void reverseWords(vector<char>& s) {
        if (s.empty()) {
            return;
        }
        reserse(s,0,s.size()-1);
        int start = 0;
        for (int i = 0; i< s.size(); i++) {
            if (s[i] == ' ') {
                reserse(s, start,i-1);
                start = i + 1;
            }
        }
        reserse(s,start,s.size()-1);
    }
};