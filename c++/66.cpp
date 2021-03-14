class Solution {
public:
    vector<int> plusOne(vector<int>& digits) {
        for (int index = digits.size()-1;index >= 0; index--) {
            digits[index] += 1;
            digits[index] = digits[index] % 10;
            if (digits[index] != 0) {
                return digits;
            }
        }
        digits.insert(digits.begin(), 1);
        return digits;
    }
};