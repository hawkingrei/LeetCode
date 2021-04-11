class Solution {
public:
    bool find(vector<int> line,int start,int end, int target) {
        if (start == end) {
            return line[start] == target;
        }
        int mid = (start+end) >>1;
        if (line[mid] == target) {
            return true;
        } else if (line[mid] > target) {
            return find(line,mid+1,end,target);
        } 
        return find(line,start,mid-1,target);
    }
    bool searchMatrix(vector<vector<int>>& matrix, int target) {
        for (vector<int>& m: matrix) {
            if (target == m[m.size()-1]) {
                return true;
            }
            if (target < m[m.size()-1]) {
                return find(m,0,m.size()-1,target);
            }
        }
        return false;
    }
};

class Solution {
public:
    bool searchMatrix(vector<vector<int>> matrix, int target) {
        auto row = upper_bound(matrix.begin(), matrix.end(), target, [](const int b, const vector<int> &a) {
            return b < a[0];
        });
        if (row == matrix.begin()) {
            return false;
        }
        --row;
        return binary_search(row->begin(), row->end(), target);
    }
};

class Solution {
public:
    bool searchMatrix(vector<vector<int>>& matrix, int target) {
        int m = matrix.size(), n = matrix[0].size();
        int low = 0, high = m * n - 1;
        while (low <= high) {
            int mid = (high - low) / 2 + low;
            int x = matrix[mid / n][mid % n];
            if (x < target) {
                low = mid + 1;
            } else if (x > target) {
                high = mid - 1;
            } else {
                return true;
            }
        }
        return false;
    }
};