// link to the problem https://pl.spoj.com/problems/AL_12_01/
#include<iostream>
#include<string>

void test_case(int no) {
	int n;
	std::cin >> n;
	std::string ops;
	std::cin >> ops;
	int sum = 0, max = 0, min = 0;
	for (char c : ops) {
		if (c == 'U') sum++; else sum--;
		if (sum > max) max = sum;
		if (sum < min) min = sum;
		if (max - min > n - 1) {
			std::cout << "NIE\n";
			return;
		}
	}

	std::cout << "TAK\n";
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