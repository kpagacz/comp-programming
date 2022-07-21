// link to the problem: https://leetcode.com/problems/reverse-linked-list/
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
  ListNode* reverseList(ListNode* head) {
    if (head == nullptr) return nullptr;
    ListNode** prev = &head;
    ListNode** pivot = &head->next;
    while (*pivot != nullptr) {
      ListNode* temp = (*pivot)->next;
      (*pivot)->next = *prev;
      *prev = *pivot;
      *pivot = temp;
    }
    return head;
  }
};

int main(int argc, char** argv) {}
