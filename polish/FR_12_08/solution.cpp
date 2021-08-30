// link to the problem https://pl.spoj.com/problems/FR_12_08/
#include<iostream>
#include<string>

void test_case(int no) {
	std::string number;
	std::cin >> number;
	int answer = 0;

	for (auto rit = number.rbegin(); rit < number.rend(); rit++) {
		if (*rit == '0') ++answer; else break;
	}
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