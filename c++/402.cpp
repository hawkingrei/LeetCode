class Solution {
public:
    string removeKdigits(string num, int k) {
        std::vector<char> stk;
        for (auto& n: num) {
            while (stk.size()>0 && stk.back() > n&& k>0) {
                stk.pop_back();
                k--;
            }
            stk.push_back(n);
        }
        for (;k> 0;k--) {
            stk.pop_back();
        }

        std::string result;
        bool is_lead = true;
        for (auto& digital: stk) {
            if (is_lead && digital == '0'){
                continue;
            } 
            is_lead = false;
            result = result + digital;
            
        }
        return result == "" ? "0": result;
    }
};