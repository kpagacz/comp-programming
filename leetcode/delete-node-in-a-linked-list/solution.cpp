// link to the problem: https://leetcode.com/problems/delete-node-in-a-linked-list/
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

// Runtime: 3 ms, faster than 99.98% of C++ online submissions for Delete Node in a Linked List.
// Memory Usage: 7.4 MB, less than 99.78% of C++ online submissions for Delete Node in a Linked List.

// Definition for singly-linked list.
struct ListNode {
  int val;
  ListNode* next;
  ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
 public:
  void deleteNode(ListNode* node) {
    node->val = node->next->val;
    node->next = node->next->next;
  }
};

int main(int argc, char** argv) {}
