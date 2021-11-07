// link to the problem  https://leetcode.com/problems/two-sum/
#include<iostream>
#include<unordered_map>
#include<vector>

class Solution {
public:
	std::vector<int> twoSum(std::vector<int>& nums, int target) {
		std::unordered_map<int32_t, int> complements;
		for (auto i = 0; i < nums.size(); i++) {
			auto found = complements.find(nums[i]);
			if (found != complements.end()) return std::vector<int> {i, found->second};
			else complements.insert({ target - nums[i], i });
		}
		return std::vector<int> {0, 0};
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