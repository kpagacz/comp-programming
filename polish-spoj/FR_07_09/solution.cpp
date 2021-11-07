// link to the problem https://pl.spoj.com/problems/FR_07_09/
#include<iostream>
#include<algorithm>
#include<string>

void test_case(int no) {
	long long number;
	std::cin >> number;
	if (number == 0) {
		std::cout << "0\n";
		return;
	}
	std::string answer = "";
	int divisor = 1;
	while (number != 0) {
		int rest = number % divisor;
		if (rest < 10) {
			answer += std::to_string(rest);
		}
		else {
			char to_add = 'A' + rest - 10;
			answer += to_add;
		}
		number /= divisor;
		++divisor;
	}

	std::reverse(answer.begin(), answer.end());
	answer.pop_back();
	std::cout << answer << "\n";
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