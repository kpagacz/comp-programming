// link to the problem: https://leetcode.com/problems/sort-list/
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
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
    ListNode* sortList(ListNode* head) {
      auto comparator = [](const auto& a, const auto& b) { return a->val < b->val; };
      std::priority_queue<ListNode*, std::vector<ListNode*>, decltype(comparator)> heap{comparator};
      while (head != nullptr) {
        heap.push(head);
        head = head->next;
      }

      if (heap.empty()) return nullptr;
      head = heap.top();
      heap.pop();
      auto it = head;
      while(!heap.empty()) {
        it->next = heap.top();
        heap.pop();
        it = it->next;
      }
      it->next = nullptr;
      return head;
    }
};

int main(int argc, char** argv) {}
