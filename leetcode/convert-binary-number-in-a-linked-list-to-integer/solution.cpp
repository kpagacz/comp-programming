// Link to the problem: https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/
/**
 * Definition for singly-linked list.
 * */
 struct ListNode {
     int val;
     ListNode *next;
     ListNode() : val(0), next(nullptr) {}
     ListNode(int x) : val(x), next(nullptr) {}
     ListNode(int x, ListNode *next) : val(x), next(next) {}
 };
#include<algorithm>
#include<string>
#include<iostream>
class Solution {
public:
    int getDecimalValue(ListNode* head) {
      std::string digits = "";
      while(head != nullptr) {
        digits += std::to_string(head->val);
        head = head->next;
      }
      return std::stoi(digits, 0, 2);
    }
};
int main () {
  ListNode *head = new ListNode(1);
  ListNode *it = head;
  for (int i = 0; i < 3; i++) {
    it->next = new ListNode(0);
    it = it->next;
  }

  Solution sol;
  std::cout << sol.getDecimalValue(head);
}
