#include <iostream>
#include <algorithm>
#include <functional>
#include <vector>
#include <array>

class Solution {
public:
    int maxEnvelopes(std::vector<std::vector<int>>& envelopes) {
        if (envelopes.empty()) {
            return 0;
        }
        int n = envelopes.size();
        std::sort(envelopes.begin(), envelopes.end(), [](const std::vector<int>& e1, const std::vector<int>& e2) {
            return e1[0] < e2[0] || (e1[0] == e2[0] && e1[1] > e2[1]);
        });
        //for (auto& vec: envelopes) {
        //        std::cout << " {" ;
        //        std::cout << vec[0] << "," << vec[1];
        //        std::cout << "} " ;
        //}
        //std::cout << std::endl;
        std::vector<int> f = { envelopes[0][1]};
        for (int i = 1; i < n; ++i) {
            if (int num = envelopes[i][1]; num > f.back()) {
                f.push_back(num);
            } else {
                auto it = std::lower_bound(f.begin(), f.end(), num);
                *it = num;
            }
            //for (auto& vec: f) {
            //    std::cout << vec ;
            //    std::cout << " " ;
            //}
            //std::cout << std::endl;
        }
        return f.size();
    }
};

int main() {
    std::vector<std::vector<int>> p{
        std::vector<int>{1,2},
        std::vector<int>{2,3},
        std::vector<int>{3,4},
        std::vector<int>{3,5},
        std::vector<int>{4,5},
        std::vector<int>{5,5},
        std::vector<int>{5,6},std::vector<int>{6,7},std::vector<int>{7,8}};
        Solution f;
    f.maxEnvelopes(p);
    return 0;
}
