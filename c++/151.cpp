class Solution {
public:
    string reverseWords(string s) {
        std::vector<string> string_vec;
        string tmp = "";
        for (char& w: s) {
            if (w == ' ') {
                if (tmp.size() != 0) {
                    string_vec.push_back(tmp);
                    tmp = "";
                }
            } else {
                tmp = tmp + w;
            }
        }
        if (tmp != "") {
            string_vec.push_back(tmp);
        }
        string result = "";
        while (string_vec.size()> 0) {
             result = result + string_vec.back();
             string_vec.pop_back();
             if (string_vec.size() != 0) {
                 result = result + " ";
             }
        }
        return result;
    }
};