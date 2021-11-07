// link to the problem https://pl.spoj.com/problems/FR_08_12/
#include<iostream>
#include<math.h>

void test_case() {
	unsigned long long n;
	int p;
	std::cin >> n >> p;
	if (n == 0) {
		std::cout << 1 << "\n";
		return;
	}
	double n_digits = std::ceil(std::log10(n) / std::log10(p));
	std::cout << n_digits << "\n";
}

int main() {
	int tests;
	std::cin >> tests;
	while (tests) {
		test_case();
		--tests;
	}
}