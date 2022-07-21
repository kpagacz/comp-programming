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
    ListNode** predecessor = &head;
    for (int i = 1; i < left; ++i) predecessor = &(*predecessor)->next;
    ListNode** pivot = &(*predecessor)->next;
    for (int i = left; i < right; ++i) {
      std::swap(*predecessor, (*pivot)->next);
      std::swap(*predecessor, *pivot);
    }

    return head;
  }
};

int main(int argc, char** argv) {}
