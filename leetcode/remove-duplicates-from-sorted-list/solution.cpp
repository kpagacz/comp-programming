// link to the problem: https://leetcode.com/problems/remove-duplicates-from-sorted-list/
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

// Runtime is random and depends on the servers having a good day. I had runtimes ranging from 10ms to 30+ms.
// Runtime: 10 ms, faster than 86.54% of C++ online submissions for Remove Duplicates from Sorted List.
// Memory Usage: 11.5 MB, less than 78.84% of C++ online submissions for Remove Duplicates from Sorted List.

// Definition for singly-linked list.
struct ListNode {
  int val;
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

class Solution {
 public:
  ListNode* deleteDuplicates(ListNode* head) {
    if (head == nullptr) return nullptr;
    while (head->next && head->val == head->next->val) head->next = head->next->next;
    head->next = deleteDuplicates(head->next);
    return head;
  }
};

int main(int argc, char** argv) {}
