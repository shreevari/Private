/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
class Solution {
public:
    ListNode* reverseList(ListNode* head) {

        if (head == NULL || head -> next == NULL)
            return head;
        ListNode* second = head -> next;
        ListNode* reversed = reverseList(head -> next);
        second -> next = head;
        head -> next = NULL;
        return reversed;
    }
};
