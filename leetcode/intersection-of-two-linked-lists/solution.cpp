// link to the problem: https://leetcode.com/problems/intersection-of-two-linked-lists/
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

// Definition for singly-linked list.
struct ListNode {
  int val;
  ListNode *next;
  ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
 public:
  ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
    ListNode *it = headA;
    while (it->next != nullptr) it = it->next;
    it->next = headB;

    ListNode *slow = headA, *fast = headA;
    while (fast != nullptr && fast->next != nullptr) {
      slow = slow->next;
      fast = fast->next->next;
      if (fast == slow) break;
    }

    if (fast != slow) {
      it->next = nullptr;
      return nullptr;
    }
    slow = headA;
    while (fast != slow) {
      fast = fast->next;
      slow = slow->next;
    }
    it->next = nullptr;
    return fast;
  }
};

int main(int argc, char **argv) {}
