// link to the problem https://leetcode.com/problems/middle-of-the-linked-list/


// struct ListNode {
//     int val;
//     ListNode *next;
//     ListNode() : val(0), next(nullptr) {}
//     ListNode(int x) : val(x), next(nullptr) {}
//     ListNode(int x, ListNode *next) : val(x), next(next) {}
// };

class Solution {
public:
    ListNode* middleNode(ListNode* head) {
      ListNode *it = head;
      int size{0};
      while(it != nullptr) {
        it = it->next;
        size++;
      }

      int middle{size / 2};
      it = head;
      while(middle) {
        it = it->next;
        --middle;
      }

      return it;
    }
};
