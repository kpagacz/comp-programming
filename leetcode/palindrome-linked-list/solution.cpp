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

// Runtime: 290 ms, faster than 73.25% of C++ online submissions for Palindrome Linked List.
// Memory Usage: 122.5 MB, less than 30.88% of C++ online submissions for Palindrome Linked List.

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
    return isPalindrome(&head, head);
  }

 private:
  bool isPalindrome(ListNode** head, ListNode* it) {
    if (it->next == nullptr) {
      int val = (*head)->val;
      (*head) = (*head)->next;
      return val == it->val;
    }

    bool answer = isPalindrome(head, it->next);
    int val = (*head)->val;
    (*head) = (*head)->next;
    return answer && val == it->val;
  }
};

int main(int argc, char** argv) {}
