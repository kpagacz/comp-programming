// link to the problem: https://leetcode.com/problems/reverse-nodes-in-k-group/
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
  ListNode* reverseKGroup(ListNode* head, int k) {
    auto end = head;
    std::stack<ListNode*> nodes;
    int counter = k;
    while (counter > 0 && end != nullptr) {
      nodes.push(end);
      end = end->next;
      counter--;
    }

    if (counter > 0) return head;

    head = nodes.top();
    nodes.pop();
    auto it = head;
    while (!nodes.empty()) {
      it->next = nodes.top();
      nodes.pop();
      it = it->next;
    }

    it->next = reverseKGroup(end, k);
    return head;
  }
};


int main(int argc, char** argv) {}
