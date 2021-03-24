class Solution {
public:
    int binarySearch(const vector<int>& accDiff, int endIndex, int target) {
        int low = 0, high = endIndex;
        while (low < high) {
            int mid = (high - low) / 2 + low;
            if (accDiff[mid] < target) {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        return low;
    }

    int equalSubstring(string s, string t, int maxCost) {
        int n = s.length();
        vector<int> accDiff(n + 1, 0);
        for (int i = 0; i < n; i++) {
            accDiff[i + 1] = accDiff[i] + abs(s[i] - t[i]);
        }
        int maxLength = 0;
        for (int i = 1; i <= n; i++) {
            int start = binarySearch(accDiff, i, accDiff[i] - maxCost);
            maxLength = max(maxLength, i - start);
        }
        return maxLength;
    }
};
