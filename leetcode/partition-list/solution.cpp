// link to the problem: https://leetcode.com/problems/partition-list/
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
  ListNode* partition(ListNode* head, int x) {
    std::vector<ListNode*> nodes;
    ListNode* it = head;
    while (it != nullptr) {
      nodes.push_back(it);
      it = it->next;
    }

    std::stable_partition(nodes.begin(), nodes.end(), [&](const auto& node) { return node->val < x; });
    for (int i = 1; i < nodes.size(); ++i) nodes[i - 1]->next = nodes[i];
    if (nodes.empty()) return nullptr;
    nodes.back()->next = nullptr;
    return nodes.front();
  }
};

int main(int argc, char** argv) {
  ListNode* head = new ListNode(1);
  head->next = new ListNode(3);
  head->next->next = new ListNode(3);
  head->next->next->next = new ListNode(2);
  Solution s;
  ListNode* new_head = s.partition(head, 3);
  while (new_head != nullptr) {
    std::cout << new_head->val;
    new_head = new_head->next;
  }
}
