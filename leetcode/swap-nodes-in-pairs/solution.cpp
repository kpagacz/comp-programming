// link to the problem: https://leetcode.com/problems/swap-nodes-in-pairs/
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
  ListNode* swapPairs(ListNode* head) {
    if (head == nullptr || head->next == nullptr) return head;
    ListNode* odd = head;
    ListNode* even = head->next;
    ListNode* odd_it = odd;
    ListNode* even_it = even;
    head = even->next;
    odd->next = nullptr;
    even->next = nullptr;
    bool odd_turn = true;
    while(head != nullptr) {
      if (odd_turn) {
        odd_it->next = head;
        odd_it = head;
      } else {
        even_it->next = head;
        even_it = head;
      }
      auto temp = head->next;
      head->next = nullptr;
      head = temp;
      odd_turn  = !odd_turn;
    }

    odd_turn = false;
    head = even;
    auto previous_odd = odd;
    while(even != nullptr) {
      auto next_even = even->next;
      auto next_odd = odd->next;
      even->next = odd;
      odd->next = next_even;
      previous_odd = odd;
      odd = next_odd;
      even = next_even;
    }
    if (odd != nullptr) previous_odd->next = odd;
    return head;
  }
};

int main(int argc, char** argv) {}
