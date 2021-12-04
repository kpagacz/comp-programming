// link to the problem
#include<iostream>

struct ListNode {
	int val;
	ListNode* next;
	ListNode() : val(0), next(nullptr) {}
	ListNode(int x) : val(x), next(nullptr) {}
	ListNode(int x, ListNode* next) : val(x), next(next) {}
};

class Solution {
public:
	Solution() = default;
	ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
		ListNode* answer = new ListNode();
		ListNode* answer_end = answer;
		answer_end->val = (l1->val + l2->val) % 10;
		int carry = (l1->val + l2->val) / 10;
		l1 = l1->next;
		l2 = l2->next;

		int l1_val, l2_val, sum;
		while (l1 != nullptr || l2 != nullptr || carry != 0) {
			answer_end->next = new ListNode();
			answer_end = answer_end->next;
			l1_val = 0; l2_val = 0;
			if (l1 != nullptr) {
				l1_val = l1->val;
				l1 = l1->next;
			}
			if (l2 != nullptr) {
				l2_val = l2->val;
				l2 = l2->next;
			}

			sum = l1_val + l2_val + carry;
			answer_end->val = sum % 10;
			carry = sum / 10;
		}
		return answer;
	}
};

int main() {
	std::ios_base::sync_with_stdio(0);
	std::cin.tie(nullptr);
	Solution sol = Solution();
}