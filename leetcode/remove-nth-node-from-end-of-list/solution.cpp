// link to the problem: https://leetcode.com/problems/remove-nth-node-from-end-of-list/
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

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
  ListNode* removeNthFromEnd(ListNode* head, int n) {
    std::vector<ListNode*> pointers;
    ListNode* it = head;
    while (it != nullptr) {
      pointers.push_back(it);
      it = it->next;
    }
    if (n == pointers.size()) return pointers[0]->next;
    auto to_remove = pointers.size() - n;
    pointers[to_remove - 1]->next = pointers[to_remove]->next;
    return pointers[0];
  }
};

int main(int argc, char** argv) {}
