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
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
 std::vector<int> vals;
 Solution(ListNode* head) {
   while(head != nullptr) {
     vals.push_back(head->val);
     head = head->next;
   }
 }

 int getRandom() {
   int chosen = rand() % vals.size();
   return vals[chosen];
 }
};

/**
 * Your Solution object will be instantiated and called as such:
 * Solution* obj = new Solution(head);
 * int param_1 = obj->getRandom();
 */
int main(int argc, char** argv) {}
