/**
 * Definition for singly-linked list.
 */
 struct ListNode {
     int val;
     ListNode *next;
     ListNode(int x) : val(x), next(nullptr) {}
 };

int get_list_length(ListNode *head) {
    int len = 0;
    while (head) {
        len++;
        head = head->next;
    }
    return len;
}

ListNode *forward_long_list(int long_len,int short_len,ListNode *head) {
    int delta = long_len - short_len;
    while (head && delta) {
        head = head->next;
        delta--;
    }
    return head;
}

class Solution {
public:
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
        int list_a_len = get_list_length(headA);
        int list_b_len = get_list_length(headB);
        if (list_a_len > list_b_len) {
            headA = forward_long_list(list_a_len,list_b_len,headA);
        } else {
            headB = forward_long_list(list_b_len,list_a_len,headB);
        }
        while (headA && headB) {
            if (headB == headA) {
                return headA;
            }
            headA = headA->next;
            headB = headB->next;            
        }
        return nullptr;
    }
};