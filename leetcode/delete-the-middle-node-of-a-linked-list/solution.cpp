// link to the problem: https://leetcode.com/problems/delete-the-middle-node-of-a-linked-list/
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

// Runtime: 1071 ms, faster than 94.61% of C++ online submissions for Delete the Middle Node of a Linked List.
// Memory Usage: 294.8 MB, less than 20.46% of C++ online submissions for Delete the Middle Node of a Linked List.

//  Definition for singly-linked list.
struct ListNode {
  int val;
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

class Solution {
 public:
  ListNode* deleteMiddle(ListNode* head) {
    ListNode *slow = head, *preslow = nullptr, *fast = head->next;
    while (fast != nullptr) {
      preslow = slow;
      slow = slow->next;
      fast = fast->next;
      if (fast) fast = fast->next;
    }
    if (slow == head) return nullptr;
    preslow->next = slow->next;
    return head;
  }
};

int main(int argc, char** argv) {}
