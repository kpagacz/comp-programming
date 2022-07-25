// link to the problem: https://leetcode.com/problems/palindrome-linked-list/
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

// Runtime: 305 ms, faster than 67.21% of C++ online submissions for Palindrome Linked List.
// Memory Usage: 123.2 MB, less than 28.04% of C++ online submissions for Palindrome Linked List.

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
  bool isPalindrome(ListNode* head) {
    if (head == nullptr) return true;
    std::stack<int> stack;
    ListNode* it = head;
    while (it != nullptr) {
      stack.push(it->val);
      it = it->next;
    }

    while (!stack.empty()) {
      if (head->val != stack.top()) return false;
      stack.pop();
      head = head->next;
    }
    return true;
  }
};

int main(int argc, char** argv) {}
