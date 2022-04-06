// link to the problem:
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

struct ListNode {
  int val;
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

class Solution {
 public:
  ListNode* swapNodes(ListNode* head, int k) {
    int total = 0;
    ListNode* it = head;
    while (it != nullptr) {
      it = it->next;
      total++;
    }

    int first = k, second = total - k + 1;
    if (2 * first == total + 1) return head;
    if (first > second) {
      int temp = first;
      first = second;
      second = temp;
    }

    ListNode *first_node = head, *first_pred = nullptr;
    int counter = 1;
    while(counter < first) {
      counter++;
      first_pred = first_node;
      first_node = first_node->next;
    }
    if (first_pred == nullptr) {
      head = first_node->next;
    } else {
      first_pred->next = first_node->next;
    }
    // std::cout << first_node->val << '\n';
    // print(head);


    ListNode* second_node = first_node->next, *second_pred = first_pred;
    counter++;
    while(counter < second) {
      counter++;
      second_pred = second_node;
      second_node = second_node->next;
    }
    if (second_pred == nullptr) {
      head = second_node->next;
    } else {
      second_pred->next = second_node->next;
    }

    // std::cout << second_node->val << '\n';
    // print(head);

    if (head == nullptr) {
      second_node->next = first_node;
      first_node->next = nullptr;
      return second_node;
    }

    if (first_pred != nullptr) {
      second_node->next = first_pred->next;
      first_pred->next = second_node;
    } else {
      second_node->next = head;
      head = second_node;
    }
    if (first_pred != second_pred) {
      first_node->next = second_pred->next;
      second_pred->next = first_node;
    } else {
      first_node->next = second_node->next;
      second_node->next = first_node;
    }
    // print(head);
    return head;
  }

  void print(ListNode* head) {
    while(head != nullptr) {
      std::cout << head->val << " ";
      head = head->next;
    }
    std::cout << '\n';
  }
};

int main(int argc, char** argv) {}
