pub fn is_subsequence(s: String, t: String) -> bool {
    if s == t {
        return true;
    }
    if s.len() > t.len() {
        return false;
    }
    let mut dp = vec![vec![false; t.len() + 1]; s.len() + 1];
    for j in 0..t.len() {
        dp[0][j] = true;
    }
    for i in 1..(s.len() + 1) {
        for j in 1..(t.len() + 1) {
            if s.chars().nth(i - 1).unwrap() == t.chars().nth(j - 1).unwrap() {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = dp[i][j - 1];
            }
        }
    }
    return dp[s.len()][t.len()];
}
/*
题解二. 后续挑战，输入量大，小写字母创建25的二维数组，存储t的坐标，这样就可以把s的判断直接转为坐标的判断，
dp[0]代表了存储了a出现在t的所有的位置,逐个字符判断s的字符顺序是否在t内，直接返回结果。
时间复杂度O(t.size()+2000)：分别为创建数组需要O(t.size()),
索引是递增的使用二分查找s的单个字符20次之内就可找到需要O(100*20)。
适用大量的输入判断子序列。

bool isSubsequence(string s, string t) {        
    vector<vector<int>>dp(26);
    int tag=-1;
    for(int i=0;i<t.size();i++)
        dp[t[i]-'a'].push_back(i);
    for(int i=0;i<s.size();i++){
        int now=s[i]-'a';
        int left=0,right=dp[now].size()-1;
        while(left<right){
            int mid=(left+right)/2;
            if(dp[now][mid]>tag)
                right=mid;
            else
                left=mid+1;
        }
        if(right<left || dp[now][left]<tag)return false;
        tag=dp[now][left];
    
    }
    return true;
}

作者：zzzzzz-5
链接：https://leetcode-cn.com/problems/is-subsequence/solution/cpan-duan-zi-xu-lie-hou-xu-tiao-zhan-by-zzzzzz-5/
来源：力扣（LeetCode）
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
*/

#[cfg(test)]
mod tests {
    use crate::is_subsequence;
    #[test]
    fn it_works() {
        assert_eq!(
            is_subsequence("abc".to_string(), "ahbgdc".to_string()),
            true
        );
    }
}
