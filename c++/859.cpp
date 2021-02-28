class Solution {
public:
    bool buddyStrings(string A, string B) {
        int aLength = A.size();
        int bLength = B.size();

        // 长度不相等, 肯定不是亲密字符串
        if (aLength != bLength) {
            return false;
        }

        // 两个字符串相等
        if (A == B) {
            // 长度1是特殊情况
            if (aLength == 1) {
                return false;
            }

            // 排序, 看看字符串中是否有字符个数大于1
            sort(A.begin(), A.end());
            auto ptr = unique(A.begin(), A.end());
            if (ptr == A.end()) {
                // 所有字符都互不相同, 不满足交换两个字符位置后 两个字符串还相等
                return false;
            }

            return true;
        }

        auto firstPos = -1;
        auto secondPos = -1;
        // 找到两个字符串中不相等的位置
        for (auto i = 0; i < aLength; ++i) {
            if (A[i] == B[i]) {
                continue;
            }

            if (firstPos == -1) {
                firstPos = i;
                continue;
            }

            if (secondPos == -1) {
                secondPos = i;
                continue;
            }

            // 有3个以上不相等的位置, 直接返回
            return false;
        }

        // 小于两个不相等的位置, 不能交换
        if (firstPos == -1 || secondPos == -1) {
            return false;
        }

        // 满足题目给出的条件
        if (A[firstPos] == B[secondPos] && A[secondPos] == B[firstPos]) {
            return true;
        }

        // 其他情况
        return false;
    }
};