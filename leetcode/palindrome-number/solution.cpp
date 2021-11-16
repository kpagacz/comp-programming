// link to the problem https://leetcode.com/problems/palindrome-number/
#include<iostream>
#include<string>

class Solution {
public:
	Solution() = default;
	bool isPalindrome(int x) {
		std::string number = std::to_string(x);
		bool isPalindrome = true;
		for (int i = 0; i < number.length() / 2; i++) {
			if (number[i] != number[number.length() - 1 - i]) {
				isPalindrome = false;
				break;
			}
		}
		return isPalindrome;
	}
};

int main() {
	std::ios_base::sync_with_stdio(0);
	std::cin.tie(nullptr);
	Solution sol = Solution();
	int x = -121;
	std::cout << sol.isPalindrome(x);
}