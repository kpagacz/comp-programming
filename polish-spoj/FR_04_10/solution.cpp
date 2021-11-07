// link to the problem https://pl.spoj.com/problems/FR_04_10/
#include<iostream>
#include<vector>
#include<algorithm>

void test_case(int no) {
	uint32_t numbers_count;
	std::cin >> numbers_count;
	std::vector<int32_t> numbers(numbers_count);
	int64_t total_sum = 0;
	for (auto& n : numbers) {
		std::cin >> n;
		total_sum += n;
	}

	uint32_t division = 0;
	int64_t running_sum = 0;
	auto count_divisions = [&](const int& n) { running_sum += n; if (2 * running_sum == total_sum) division++; };
	std::for_each(numbers.begin(), numbers.end() - 1, count_divisions);

	std::cout << division << '\n';
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