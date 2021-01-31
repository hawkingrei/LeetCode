struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
    ListNode* reverseBetween(ListNode* head, int m, int n) {
        int change_len = n - m + 1;
        ListNode *pre_head = nullptr;
        ListNode *result = head;
        while (head && --m) {
            pre_head = head;
            head = head->next;
        }
        ListNode *modify_list_tail = head;
        ListNode *new_head = nullptr;
        while (head && change_len) {
            ListNode *next = head->next;
            head->next = new_head;
            new_head = head;
            head = next;
            change_len--;
        }
        modify_list_tail->next=head;
        if (pre_head) {
            pre_head->next = new_head;
        } else {
            result = new_head;
        }
        return result;
    }
};