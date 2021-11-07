// link to the problem https://pl.spoj.com/problems/WWO_01_11/
#include<iostream>
#include<string>

void test_case(int no) {
	std::string base_3;
	int n;
	std::cin >> n;
	char digits[3] = { '0', '1', '2' };
	while (n) {
		base_3 = digits[n % 3] + base_3;
		n /= 3;
	}

	int non_zeros = 0;
	bool surplus = false;
	for(char c : base_3) {
		switch (c) {
		case '0':
			if (surplus) {
				++non_zeros;
			}
			break;
		case '1':
			non_zeros++;
			break;
		case '2':
			non_zeros++;
			surplus = true;
			break;
		}
	}
	std::cout << non_zeros << '\n';
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