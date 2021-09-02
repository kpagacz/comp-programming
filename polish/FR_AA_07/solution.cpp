// link to the problem https://pl.spoj.com/problems/FR_AA_07/
#include<iostream>

void test_case(int no) {
	int k, n;
	std::cin >> k >> n;
	for (int i = 0; i < k - 1; i++) {
		std::cout << n / k - k / 2 + i + (k % 2 == 0)<< " ";
	}
	std::cout << n / k - k / 2 + k - 1 + (k % 2 == 0) << "\n";
}

int main() {
	std::ios_base::sync_with_stdio(0);
	std::cin.tie(nullptr);
	int tests;
	tests = 1;
	while (tests) {
		test_case(tests);
		--tests;
	}
}