// link to the problem: https://leetcode.com/problems/merge-two-sorted-lists/
#include<iostream>
#include<vector>
#include<array>
#include<string>
#include<algorithm>
#include<numeric>
#include<sstream>
#include<iterator>

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    ListNode* mergeTwoLists(ListNode* list1, ListNode* list2) {
      ListNode* answer = new ListNode();
      ListNode* answerit = answer;
      ListNode *it1 = list1, *it2 = list2;
      while(it1 != nullptr && it2 != nullptr) {
        if (it1->val < it2->val) {
          answerit->next = it1;
          it1 = it1->next;
        } else {
          answerit->next = it2;
          it2 = it2->next;
        }
        answerit = answerit->next;
      }
      if (it1 == nullptr) {
        answerit->next = it2;
      } else {
        answerit->next = it1;
      }

      return answer->next;
    }

    ListNode* recMergeTwoLists(ListNode* list1, ListNode* list2) {
      if (list1 == nullptr) return list2;
      if (list2 == nullptr) return list1;

      ListNode* newHead = nullptr;
      if (list1->val < list2->val) {
        newHead = list1;
        newHead->next = recMergeTwoLists(list1->next, list2);
      } else {
        newHead = list2;
        newHead->next = recMergeTwoLists(list1, list2->next);
      }

      return newHead;
    }
};

int main(int argc, char** argv) {

}
