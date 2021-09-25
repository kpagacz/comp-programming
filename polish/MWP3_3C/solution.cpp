// link to the problem https://pl.spoj.com/problems/MWP3_3C/
#include<iostream>
#include<vector>

void test_case(int no) {
	int lines;
	std::cin >> lines;
	uint32_t nim_sum = 0;
	int count;
	for (int i = 0; i < lines; i++){
		std::cin >> count;
		nim_sum ^= count;
	}

	if (nim_sum == 0) {
		std::cout << 2 << '\n';
	}
	else {
		std::cout << 1 << '\n';
	}
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