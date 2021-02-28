class Solution 
{
public:
    vector<int> findSubstring(string s, vector<string>& words) 
    {           //滑动窗口 + shell希尔排序 + 哈希统计
        int n = s.size();
        if (n == 0 || words.size() == 0)
            return vector<int> {};
        int word_num = words.size();            //单词个数
        int one_w_len = words[0].size();        //一个单词的长度

        unordered_map<string, int> word_cnt;    //统计每个单词出现的freq
        for (string & w: words) {
            word_cnt[w] += 1;
        }
            
        
        vector<int> res;
        for (int i = 0; i < one_w_len; i ++)    //每次的起点  类似shell希尔排序的起点
        {                                       //delta = 一个单词的长度
            int cur_w_num = 0;                  //滑动窗口内单词的个数
            int L = i,  R = i;                  //LR指针
            unordered_map<string, int> cur_w_cnt;   //统计滑动窗口内 每个单词的freq
            while (R + one_w_len <= s.size())        //R的范围
            {
                string w = s.substr(R, one_w_len);
                R += one_w_len;                     //R右移
                if (word_cnt.count(w) == 0)         //不是有效的单词
                {                                   //突然冒出一个傻子 前面就全废了
                    L = R;
                    cur_w_cnt.clear();
                    cur_w_num = 0;
                }
                else
                {
                    cur_w_cnt[w] += 1;
                    cur_w_num += 1;
                    while (cur_w_cnt[w] > word_cnt[w])  //处理的L标准模板
                    {
                        string L_w = s.substr(L, one_w_len);
                        L += one_w_len;
                        cur_w_cnt[L_w]--;
                        cur_w_num--;
                    }
                    if (cur_w_num == word_num)          //若单词个数对上了，就记录下来
                        res.push_back(L);
                }
            }
        }
        return res;
    }
};