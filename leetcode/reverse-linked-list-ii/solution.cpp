// link to the problem: https://leetcode.com/problems/reverse-linked-list-ii/
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
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

class Solution {
 public:
  ListNode* reverseBetween(ListNode* head, int left, int right) {
    ListNode* predecessor = nullptr;
    ListNode* it = head;
    int counter = 1;
    while (counter < left && it != nullptr) {
      predecessor = it;
      it = it->next;
      ++counter;
    }

    std::stack<ListNode*> stack;
    while (it != nullptr && counter <= right) {
      stack.push(it);
      it = it->next;
      ++counter;
    }

    if (predecessor == nullptr)
      head = stack.top();
    else
      predecessor->next = stack.top();

    while (stack.size() > 1) {
      auto& top = stack.top();
      stack.pop();
      top->next = stack.top();
    }
    stack.top()->next = it;

    return head;
  }
};

int main(int argc, char** argv) {}
