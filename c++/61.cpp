/*
 * @lc app=leetcode.cn id=61 lang=cpp
 *
 * [61] 旋转链表
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    ListNode* rotateRight(ListNode* head, int k) {
        if (head == nullptr) {
            return head;
        }
        if (k == 0) {
            return head;
        }
        ListNode* fast = head;
        ListNode* slow = head;
        int length = 0;
        while (fast) {
            length++;
            fast = fast->next;
        }
        k = k % length;
        fast = head;
        for (int i = 0; i < k; i++) {
            fast=fast-> head;
        }
        while (fast->next) {
            fast = fast->next;
            slow = slow->next;
        }
        fast->next = head;
        head = slow->next;
        slow->next = nullptr;
        return head;
    }
};
// @lc code=end

