class Solution {
public:
    string largestNumber(vector<int>& nums) {
        if(nums.empty()) return "";
        if(nums.size() == 1) return to_string(nums[0]);

        sort(nums.begin(), nums.end(), comparison); // 调用STL中的sort函数
        string result = "";
        for(int i : nums)
        {
            result += to_string(i);
        }
        if(result[0] == '0') return "0"; // 特殊case，全是0的时候应该输出0而不是00000
        return result;
    }
    static bool comparison(const int& a, const int& b)
    {
        // 注：a和b也可以不用传引用（即 &）
        // 注：此处要用static，因为std::sort是属于全局的，无法调用非静态成员函数，而静态成员函数或全局函数是不依赖于具体对象，可以独立访问。
        // 也可以把comparison这个函数放在Solution这个class的外面，但是记住一定要放在整个class的上面而不能是下面。
        // 不然代码里调用sort函数时会找不到comparison，而导致报错。
        string concatenation1 = to_string(a) + to_string(b);
        string concatenation2 = to_string(b) + to_string(a);
        
        return concatenation1 > concatenation2;
    }
};