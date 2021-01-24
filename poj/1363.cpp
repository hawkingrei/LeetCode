#include <stack>
#include <queue>

bool check_is_valid_order(std::queue<int> &order) {
    std::stack<int> s;
    int n = order.size();
    for (int i = 1; 1<=n; i++) {
        s.push(i);
        while (!s.empty() && order.front() == s.top()) {
            s.pop();
            order.pop();
        }
    }
    if (!s.empty()) {
        return false;
    }
    return true;
}
