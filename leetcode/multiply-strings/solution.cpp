// link to the problem https://leetcode.com/problems/multiply-strings/
#include<iostream>
#include<string>
#include<vector>
#include<algorithm>

class Solution {
public:
	std::string multiply(std::string num1, std::string num2) {
		std::vector<int> answer(num1.size() + num2.size(), 0);
		for (int i = 0; i < num2.size(); ++i) {
			for (int j = 0; j < num1.size(); ++j) {
				answer[j + i] += (num1[num1.size() - j - 1] - '0') * (num2[num2.size() - i - 1] - '0');
			}
		}
		for (int i = 0; i < answer.size() - 2; i++) {
			answer[i + 1] += answer[i] / 10;
			answer[i] = answer[i] % 10;
		}
		std::reverse(answer.begin(), answer.end());
		std::string digits;
		for (const auto& digit : answer) digits += std::to_string(digit);
		if (digits.find_first_not_of('0') == digits.npos) return "0";
		else digits.erase(0, digits.find_first_not_of('0'));
		return digits;
	}
};

void test_case(const int& test) {
	std::string a, b;
	std::cin >> a >> b;
	Solution sol;
	sol.multiply(a, b);
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