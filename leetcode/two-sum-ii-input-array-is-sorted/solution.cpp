// link to the problem https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
#include<iostream>
#include<vector>

class Solution {
public:
	std::vector<int> twoSum(std::vector<int>& numbers, int target) {
		std::vector<int>::iterator forward_it = numbers.begin(), reverse_it = numbers.end() - 1;
		while (forward_it < reverse_it) {
			if (*forward_it + *reverse_it == target) {
				int for_ind = forward_it - numbers.begin() + 1;
				int rev_ind = reverse_it - numbers.begin() + 1;
				return std::vector<int> {for_ind, rev_ind};
			}

			if (*forward_it + *reverse_it > target) {
				--reverse_it;
			}
			else {
				++forward_it;
			}
		}
		return { 0, 0 };
	}
};

void test_case(int no) {
	
}

int main() {
	std::ios_base::sync_with_stdio(0);
	std::cin.tie(nullptr);
	int tests;
	std::cin >> tests;
	while (tests) {
		test_case(tests);
		--tests;
	}
}