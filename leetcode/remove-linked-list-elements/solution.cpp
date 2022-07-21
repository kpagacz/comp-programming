// link to the problem: https://leetcode.com/problems/remove-linked-list-elements/
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

// Runtime: 41 ms, faster than 35.63% of C++ online submissions for Remove Linked List Elements.
// Memory Usage: 15 MB, less than 46.79% of C++ online submissions for Remove Linked List Elements.

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
  ListNode* removeElements(ListNode* head, int val) {
    ListNode** it = &head;
    while (*it != nullptr)
      if ((*it)->val == val)
        *it = (*it)->next;
      else
        it = &((*it)->next);

    return head;
  }
};

int main(int argc, char** argv) {}
