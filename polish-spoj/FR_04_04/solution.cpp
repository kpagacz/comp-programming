// link to the problem https://pl.spoj.com/problems/FR_04_04/
#include<string>
#include<iostream>

void test_case() {
	std::string sum;
	std::getline(std::cin, sum);
	bool sign_before, symbol_before;
	sign_before = true;

	for (char c : sum) {
		if (c == '|' && sign_before) {
			std::cout << '(';
		}
		else if (c == '|' && symbol_before) {
			std::cout << ')';
		}
		else if (c == '+' || c == '-') {
			sign_before = true;
			symbol_before = false;
			std::cout << c;
		}
		else {
			sign_before = false;
			symbol_before = true;
			std::cout << c;
		}
	}
	std::cout << '\n';
}

int main() {
	int tests;
	std::cin >> tests;
	std::cin.ignore();
	for (int i = 0; i < tests; ++i) {
		test_case();
	}
}