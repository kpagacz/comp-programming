// link to the problem: https://leetcode.com/problems/merge-k-sorted-lists/
#include <algorithm>
#include <array>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
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
  ListNode* mergeKLists(std::vector<ListNode*>& lists) {
    auto compareNodes = [](ListNode* a, ListNode* b) { return a->val > b->val; };
    std::priority_queue min_heap(compareNodes, std::vector<ListNode*>());
    std::for_each(lists.begin(), lists.end(), [&](auto list) {
      if (list != nullptr) min_heap.push(list);
    });

    if (min_heap.empty()) return nullptr;
    auto answer = min_heap.top();
    auto it = answer;
    while (!min_heap.empty()) {
      it->next = min_heap.top();
      it = it->next;
      min_heap.pop();
      if (it->next != nullptr) min_heap.push(it->next);
    }

    return answer->next;
  }

  ListNode* mergeKListsUglierButFaster(std::vector<ListNode*>& lists) {
    auto compareNodes = [](ListNode* a, ListNode* b) { return a->val > b->val; };
    std::priority_queue min_heap(compareNodes, std::vector<ListNode*>());
    std::for_each(lists.begin(), lists.end(), [&](auto list) {
      if (list != nullptr) min_heap.push(list);
    });

    if (min_heap.empty()) return nullptr;
    auto answer = min_heap.top();
    auto it = answer;
    min_heap.pop();
    if (it->next != nullptr) min_heap.push(it->next);
    while (!min_heap.empty()) {
      it->next = min_heap.top();
      it = it->next;
      min_heap.pop();
      if (it->next != nullptr) min_heap.push(it->next);
    }

    return answer;
  }
};

int main(int argc, char** argv) {}
