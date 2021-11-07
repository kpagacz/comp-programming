// link to the problem https://pl.spoj.com/problems/FR_05_07/
#include<iostream>
#include<vector>
#include<algorithm>
#include<numeric>

void test_case(int no) {
	int n;
	std::cin >> n;
	int size;
	int number;
	std::cin >> number;
	size = number;
	std::vector<int> heights(size + 1, 0);
	heights[number]++;
	for (int i = 0; i < n - 1; ++i) {
		std::cin >> number;
		heights[number]++;
	}

	std::partial_sum(heights.rbegin(), heights.rend(), heights.rbegin());
	for (auto it = heights.begin() + 1; it != heights.end(); it++) {
		std::cout << *it << " ";
	}
	std::cout << "\n";
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