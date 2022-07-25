// link to the problem: https://leetcode.com/problems/linked-list-cycle
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
    ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
public:
    bool hasCycle(ListNode *head) {
        return hasCycle(head, head);
    }
    bool hasCycle(ListNode* head, ListNode* it) {
        if (it == nullptr) return false;
        if (it->next == head) return true;
        ListNode* tmp = it->next;
        it->next = head;
        return hasCycle(head, tmp);
    }
};

int main(int argc, char** argv) {}
