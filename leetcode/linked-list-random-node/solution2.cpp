// link to the problem: https://leetcode.com/problems/linked-list-random-node/
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
#include <stack>
#include <stdexcept>
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
  ListNode* head;
  Solution(ListNode* head) {
    this->head = head;
  }
  int getRandom() {
    std::vector<int> reservoir;
    auto it = head;
    int count = 0;
    while (reservoir.size() < 30 && it != nullptr) {
      reservoir.push_back(it->val);
      it = it->next;
      count++;
    }

    double expectation = std::exp(std::log(1.0 * rand() / RAND_MAX) / 30);
    while (it != nullptr) {
      int to_skip = std::floor(std::log(1.0 * rand() / RAND_MAX) / std::log(1 - expectation)) + 1;
      while(to_skip > 0 && it != nullptr) it = it->next;

      if (it != nullptr) {
        int random = rand() % count;
        reservoir[random] = it->val;
        it = it->next;
        expectation *= std::exp(std::log(1.0 * rand() / RAND_MAX) / 30);
      }
    }

    return reservoir[rand() % reservoir.size()];
  }
};

/**
 * Your Solution object will be instantiated and called as such:
 * Solution* obj = new Solution(head);
 * int param_1 = obj->getRandom();
 */
int main(int argc, char** argv) {
}
