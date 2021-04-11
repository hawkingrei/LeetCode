class Solution {
public:
    string simplifyPath(string path) {
        std::vector<string> result_stack;
        string tmp = "";
        for (char& w: path) {
            if (w == '/') {
                if (tmp == "") {
                    continue;
                } 
                if (tmp == ".") {
                    tmp = "";
                    continue;
                }
                if (tmp == "..") {
                    tmp = "";
                    if (!result_stack.empty()) {
                        result_stack.pop_back();
                    }
                    continue;
                }
                result_stack.push_back(tmp);
                tmp = "";
                continue;
            }
            tmp = tmp+w;
        }
        if (!tmp.empty()) {
            if (tmp == "..") {
                if (!result_stack.empty()) {
                    result_stack.pop_back();
                }

            } else 
            if (tmp != "" || tmp != ".") {
                result_stack.push_back(tmp);
            }
        }
        if (result_stack.size() == 0) {
            return "/";
        }
        string result = "";
        for (auto& w: result_stack) {
            result = result + "/" + w;
        }
        return result;
    }
};