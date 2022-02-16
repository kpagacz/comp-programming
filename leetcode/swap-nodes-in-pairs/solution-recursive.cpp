// a super slick solution I have looked up. Much to learn from this.
// Pointer-pointer pp points to the pointer to the current node. So at first, pp points to head, and later it points to
// the next field of ListNodes. Additionally, for convenience and clarity, pointers a and b point to the current node
// and the next node.
//  We need to go from *pp == a -> b -> (b->next) to *pp == b -> a -> (b->next). The first three lines
// inside the loop do that, setting those three pointers (from right to left). The fourth line moves pp to the next
// pair.
struct ListNode {
  int val;
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

class Solution {
 public:
  ListNode* swapPairs(ListNode* head) {
    if (head == nullptr || head->next == nullptr) return head;
    auto next = head->next;
    auto nextnext = head->next->next;
    head->next = swapPairs(nextnext);
    next->next = head;
    return head;
  }
};
